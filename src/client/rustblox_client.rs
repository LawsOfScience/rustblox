use crate::error::{ClientError, RequestError, RobloxApiError, RobloxApiErrors};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};

pub(crate) struct RequestComponents {
    pub(crate) needs_auth: bool,
    pub(crate) method: Method,
    pub(crate) url: String,
    pub(crate) headers: Option<HeaderMap>,
    pub(crate) body: Option<String>,
}

/// The Rustblox client. All functions necessary to contact endpoints
/// are contained within `impl`s. Eventually, users will be able to control
/// which functions they want with crate features.
pub struct RustbloxClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) roblox_cookie: Option<String>,
    pub(crate) auto_reauth: bool,

    // So, why Arc<RwLock<Option<String>>>? Let's break this down.
    // TL;DR: session reusability within one client.
    //
    // So, imagine you have an async API that needs your Client in two or more
    // endpoints. You COULD make a new Client for each endpoint, but if your
    // client needs authentication, you might end up getting ratelimited as you
    // continually try to fetch an X-CSRF-TOKEN.
    // This is a solution -- using one Client instance and sharing the csrf token
    // with an Arc<RwLock<Option<String>>> to avoid needing to fetch it all the time.
    // - Arc: for sharing the memory
    // - RwLock: to gate access. Reads are also more common than writes since
    //           csrf tokens aren't extremely ephemeral.
    //   - Also prevents make_request() needing to take &mut self, which would
    //     effectively only permit one request to be processed at a time (subpar!)
    //     - And yes, I do believe the &mut self issue still appears if I instead
    //       make the user have their own Arc<Mutex<RustbloxClient>>
    // - Option<String>: if you aren't using authentication, you don't need a csrf token
    //
    // I am open to being corrected if this is a terrible solution and
    // I should do it another way.
    pub(crate) csrf_token: Arc<RwLock<Option<String>>>,
}

impl RustbloxClient {
    #[must_use]
    pub fn csrf_token(&self) -> Option<String> {
        match self.csrf_token.read() {
            Ok(inner) => inner.clone(),
            Err(why) => {
                warn!("The CSRF RwLock is poisoned:\n{why}");
                None
            }
        }
    }

    /// Returns a boolean representing this [`RustbloxClient`]'s authentication status.
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        let csrf = match self.csrf_token.read() {
            Ok(inner) => inner.is_some(),
            Err(why) => {
                warn!("The CSRF RwLock is poisoned:\n{why}");
                false
            }
        };
        self.roblox_cookie().is_some() && csrf
    }

    /// Logs the client in.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///     - The login process failed.
    ///     - The provided cookie is invalid.
    ///     - An `x-csrf-token` could not be obtained.
    ///     - An invalid `x-csrf-token` was obtained.
    ///
    /// # Panics
    ///
    /// This function cannot panic.
    pub async fn login(&self) -> Result<(), ClientError> {
        // Initial connection test can come first
        // and return early if no cookie is set
        self.reqwest_client
            .get("https://roblox.com")
            .send()
            .await
            .map_err(|e| ClientError::LoginFailed(format!("Could not reach roblox.com ({e})")))?;

        debug!("Successfully pinged Roblox");

        if self.roblox_cookie().is_none() {
            warn!(
                "[WARN/LOGIN]: Please be advised that you do not have a .ROBLOSECURITY cookie set.\
                \nYou will not be able to use functions that require this."
            );
            return Ok(()); // Return early
        }

        let auth_response = self
            .reqwest_client
            .post("https://auth.roblox.com/v2/logout")
            .header("Content-Length", 0)
            .header("Cookie", self.roblox_cookie().unwrap())
            .send()
            .await
            .map_err(|e| {
                ClientError::LoginFailed(format!("Authentication request error: \n{e}"))
            })?;

        (auth_response.status() != 403)
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), Err)?;

        std::ops::Not::not(auth_response.headers().contains_key("x-csrf-token"))
            .then_some(ClientError::LoginFailed(
                "No x-csrf-token was given by Roblox".to_string(),
            ))
            .map_or(Ok(()), Err)?;

        let csrf_from_headers = auth_response.headers().get("x-csrf-token").unwrap().clone();

        let csrf_string = csrf_from_headers
            .to_str()
            .map_err(|e| ClientError::LoginFailed(format!("Failed to parse CSRF token:\n{e}")))?
            .to_string();

        let mut csrf = match self.csrf_token.write() {
            Ok(inner) => inner,
            Err(why) => {
                warn!("The CSRF RwLock is poisoned:\n{why}");
                return Err(ClientError::LoginFailed("The CSRF RwLock is poisoned".into()));
            }
        };
        *csrf = Some(csrf_string);

        Ok(())
    }

    /// Makes a request to the Roblox API.
    /// If the endpoint returns a 403 error and this function determines that the Client's
    /// `x-csrf-token` is invalid, it will attempt to reauthenticate itself if `Client.auto_reauth`
    /// is set to `true`. This value can be set while building the client.
    /// The Client will only attempt reauthentication once (see the definition of insanity
    /// for why).
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - An `x-csrf-token` could not be obtained.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - You attempt to contact an endpoint that requires authentication while unauthenticated.
    /// - The endpoint responds with an error.
    /// - Your `.ROBLOSECURITY` cookie or `x-csrf-token` are invalid and automatic reauthentication failed
    /// or was not enabled. In either case, you will get a [`RequestError::ReauthenticationFailed`].
    #[async_recursion::async_recursion]
    pub(crate) async fn make_request<T>(
        &self,
        components: RequestComponents,
        tried_reauth: bool,
    ) -> Result<T, RequestError>
    where
        T: DeserializeOwned,
    {
        (components.needs_auth && self.roblox_cookie().is_none())
            .then_some(RequestError::NotAuthenticated)
            .map_or(Ok(()), Err)?;

        let mut request = self
            .reqwest_client
            .request(components.method.clone(), components.url.clone());
        if components.needs_auth {
            request = request
                .header("Cookie", self.roblox_cookie().unwrap())
                .header("x-csrf-token", self.csrf_token().unwrap());
        }

        if components.headers.is_some() {
            request = request.headers(components.headers.clone().unwrap());
        }
        if components.body.is_some() {
            request = request.body(components.body.clone().unwrap());
        }

        let response = request
            .send()
            .await
            .map_err(|e| RequestError::RequestError(components.url.clone(), e.to_string()))?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            return if response.status().is_client_error() {
                if status_code == 401 {
                    // Bad cookie
                    return Err(RequestError::ExpiredCookie);
                }

                // Have to parse as serde_json::Value because SPECIFICALLY
                // 403 Token Validation Failed has different JSON than the
                // rest of the errors
                let err_body_text = response.text().await.map_err(|e| {
                    RequestError::RequestError(
                        components.url.clone(),
                        format!("Failed to get error response body:\n{}", e)
                    )
                })?;
                let raw_err_body = serde_json::from_str::<serde_json::Value>(err_body_text.as_str()).map_err(|e| {
                    RequestError::RequestError(
                        components.url.clone(),
                        format!("Couldn't parse error body json:\n{}Response body:\n{}", e, err_body_text),
                    )
                })?;

                if status_code == 403 {
                    // We need to find out if the Roblox API wants us to reauthenticate or if
                    // the error is for a different reason
                    if raw_err_body.get("code").is_some() {
                        // This means that we *probably* have a 403 Token Validation Failed
                        // since the json is just { code: number, message: string }
                        if raw_err_body.get("code").unwrap() != 0 {
                            return Err(RequestError::RequestError(
                                components.url.clone(),
                                format!("Unknown 403 error:\n{}", raw_err_body)
                            ));
                        }

                        // Now we definitely have a 403 Token Validation Failed
                        if self.auto_reauth && !tried_reauth {
                            self.login()
                                .await
                                .map_err(|e| RequestError::ReauthenticationFailed(e.to_string()))?;

                            return self.make_request::<T>(components, true).await;
                        }
                        return Err(RequestError::ReauthenticationFailed(
                            "Automatic reauthentication either not enabled or already tried"
                                .to_string(),
                        ));
                    }
                }

                let err_body = serde_json::from_value::<RobloxApiErrors>(raw_err_body.clone())
                    .map_err(|e|
                        RequestError::RequestError(
                            components.url.clone(),
                            format!("Couldn't parse error body json as RobloxApiErrors:\n{}\nResponse body:\n{}", e, raw_err_body),
                    ))?;

                Err(RequestError::ClientError(
                    components.url,
                    status_code,
                    err_body,
                ))
            } else if response.status().is_server_error() {
                Err(RequestError::ServerError(status_code))
            } else {
                let body = response.text().await.map_err(|e| {
                    RequestError::RequestError(
                        components.url.clone(),
                        format!("Couldn't parse body as string:\n{}", e),
                    )
                })?;

                let unknown_error = RobloxApiError {
                    code: -999,
                    message: body,
                };
                let error_struct = RobloxApiErrors {
                    errors: vec![unknown_error],
                };
                Err(RequestError::ClientError(
                    components.url,
                    status_code,
                    error_struct,
                ))
            };
        }

        let response_data = response
            .json::<T>()
            .await
            .map_err(|e| RequestError::RequestError(components.url, e.to_string()))?;

        Ok(response_data)
    }

    /// Returns the roblox cookie of this [`RustbloxClient`].
    #[must_use]
    pub fn roblox_cookie(&self) -> Option<&String> {
        self.roblox_cookie.as_ref()
    }
}
