use crate::client::RustbloxClient;
use crate::error::ClientError;

pub struct RustbloxClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    roblox_cookie: Option<String>,
    automatic_reauth: bool,
}

impl Default for RustbloxClientBuilder {
    fn default() -> Self {
        RustbloxClientBuilder::new()
    }
}

#[inline]
fn get_user_agent() -> String {
    format!("Rustblox v{}", env!("CARGO_PKG_VERSION"))
}

impl RustbloxClientBuilder {
    /// Sets whether or not the [`RustbloxClient`] should attempt to reauthenticate itself if
    /// Roblox returns a 403 status code from an endpoint that requires authentication. Defaults to `true`.
    ///
    /// # Errors
    /// This function cannot error.
    #[inline]
    pub fn automatic_reauth(mut self, automatic_reauth: bool) -> Self {
        self.automatic_reauth = automatic_reauth;
        self
    }

    /// Attempts to use a `RustbloxClientBuilder` to construct a `RustbloxClient`. This method will fail if reqwest fails to build a client.
    ///
    /// # Errors
    ///
    /// This function returns an error if the reqwest client builder returns an error.
    pub fn build(self) -> Result<RustbloxClient, ClientError> {
        let built_client = self
            .reqwest_builder
            .build()
            .map_err(|e| ClientError::ReqwestBuildError(e.to_string()))?;

        Ok(RustbloxClient {
            reqwest_client: built_client,
            roblox_cookie: self.roblox_cookie,
            csrf_token: None,
            automatic_reauth: self.automatic_reauth
        })
    }

    /// Inserts a token into a `RustbloxClientBuilder`.
    ///
    /// # Errors
    ///
    /// This function returns an error if the cookie provided is invalid.
    pub fn insert_cookie(mut self, cookie: &str) -> Result<Self, ClientError> {
        // All .ROBLOSECURITY cookies should start with this
        // unless Roblox just decides to randomly change it some day
        // ...which they might

        (!(cookie.starts_with("_|WARNING")))
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), Err)?;

        let formatted_cookie = format!(".ROBLOSECURITY={cookie}");

        self.roblox_cookie = Some(formatted_cookie);

        Ok(self)
    }

    /// Creates a new `RustbloxClientBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            reqwest_builder: reqwest::ClientBuilder::new().user_agent(get_user_agent()),
            roblox_cookie: None,
            automatic_reauth: true,
        }
    }
}
