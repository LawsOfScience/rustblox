use crate::client::RustbloxClient;
use crate::error::ClientError;

pub struct RustbloxClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    roblox_cookie: Option<String>,
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
            .map_or(Ok(()), |e| Err(e))?;

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
        }
    }

    /// Creates a new `RustBloxClientBuilder` with a token.
    ///
    /// # Errors
    ///
    /// This function returns an error if the cookie provided is invalid.  
    pub fn with_cookie(cookie: &str) -> Result<Self, ClientError> {
        (!(cookie.starts_with("_|WARNING")))
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), |e| Err(e))?;

        let formatted_cookie = format!(".ROBLOSECURITY={cookie}");

        Ok(Self {
            reqwest_builder: reqwest::ClientBuilder::new().user_agent(get_user_agent()),
            roblox_cookie: Some(formatted_cookie),
        })
    }
}
