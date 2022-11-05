use crate::client::RustbloxClient;
use crate::error::ClientError;

pub struct RustbloxClientBuilder {
    reqwest_builder: reqwest::ClientBuilder,
    roblox_token: Option<String>,
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
    /// Attempts to use a `RustbloxClientBuilder` to construct a RustbloxClient. This method will fail if reqwest fails to build a client.
    pub fn build(self) -> Result<RustbloxClient, ClientError> {
        let built_client = self
            .reqwest_builder
            .build()
            .map_err(|e| ClientError::ReqwestBuildError(e.to_string()))?;

        Ok(RustbloxClient {
            reqwest_client: built_client,
            roblox_token: self.roblox_token,
            csrf_token: None,
        })
    }

    /// Inserts a token into a `RustbloxClientBuilder`. Fails if an invalid token is entered.
    pub fn insert_token(mut self, token: &str) -> Result<Self, ClientError> {
        // All .ROBLOSECURITY cookies should start with this
        // unless Roblox just decides to randomly change it some day
        // ...which they might

        (!(token.starts_with("_|WARNING")))
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), |e| Err(e))?;

        let formatted_token = format!(".ROBLOSECURITY={token}");

        self.roblox_token = Some(formatted_token);

        Ok(self)
    }

    /// Creates a new `RustbloxClientBuilder`.
    pub fn new() -> Self {
        Self {
            reqwest_builder: reqwest::ClientBuilder::new().user_agent(get_user_agent()),
            roblox_token: None,
        }
    }

    /// Creates a new `RustBloxClientBuilder` with a token. Fails if an invalid token is entered.
    pub fn with_token(token: &str) -> Result<Self, ClientError> {
        (!(token.starts_with("_|WARNING")))
            .then_some(ClientError::InvalidCookie)
            .map_or(Ok(()), |e| Err(e))?;

        let formatted_token = format!(".ROBLOSECURITY={token}");

        Ok(Self {
            reqwest_builder: reqwest::ClientBuilder::new().user_agent(get_user_agent()),
            roblox_token: Some(formatted_token),
        })
    }
}
