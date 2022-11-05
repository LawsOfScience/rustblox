use crate::error::{ClientError, RequestError};
use log::{debug, warn};

pub struct RustbloxClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) roblox_cookie: Option<String>,
    pub(crate) csrf_token: Option<String>,
}

impl RustbloxClient {
    #[must_use]
    pub fn csrf_token(&self) -> Option<&String> {
        self.csrf_token.as_ref()
    }

    /// Returns a boolean representing this [`RustbloxClient`]'s authentication status.
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.roblox_cookie().is_some() && self.csrf_token.is_some()
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
    pub async fn login(&mut self) -> Result<(), ClientError> {
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

        self.csrf_token = Some(csrf_string);

        Ok(())
    }

    /// Makes a request to the Roblox API.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     - An `x-csrf-token` could not be obtained.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    ///     - You attempt to contact an endpoint that requires authentication while unauthenticated
    ///     - The endpoint responds with an error.
    pub(crate) async fn make_request(
        &self,
        url: String,
        method: reqwest::Method,
        needs_auth: bool,
    ) -> Result<reqwest::Response, RequestError> {
        (needs_auth && self.roblox_cookie().is_none())
            .then_some(RequestError::NotAuthenticated)
            .map_or(Ok(()), Err)?;

        let mut request = self.reqwest_client.request(method, &url);

        if needs_auth {
            request = request
                .header("Cookie", self.roblox_cookie().unwrap())
                .header("x-csrf-token", self.csrf_token().unwrap());
        }

        let response = request
            .send()
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        Ok(response)
    }

    /// Returns the roblox cookie of this [`RustbloxClient`].
    #[must_use]
    pub fn roblox_cookie(&self) -> Option<&String> {
        self.roblox_cookie.as_ref()
    }
}
