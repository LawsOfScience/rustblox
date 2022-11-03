use crate::client::RustbloxClient;
use crate::error::ClientError;

pub struct RustbloxClientBuilder {
    error: Option<ClientError>,
    reqwest_builder: reqwest::ClientBuilder,
    roblox_token: Option<String>,
}

impl Default for RustbloxClientBuilder {
    fn default() -> Self {
        RustbloxClientBuilder::new()
    }
}

impl RustbloxClientBuilder {
    pub fn new() -> Self {
        RustbloxClientBuilder {
            error: None,
            reqwest_builder: reqwest::ClientBuilder::new().user_agent("Rustblox v0.0.1"),
            roblox_token: None,
        }
    }

    pub fn build(self) -> Result<RustbloxClient, ClientError> {
        if let Some(error) = self.error {
            return Err(error);
        }
        let built_client = match self.reqwest_builder.build() {
            Ok(reqwest_client) => reqwest_client,
            Err(err) => {
                return Err(ClientError::ReqwestBuildError(err.to_string()));
            }
        };

        Ok(RustbloxClient {
            reqwest_client: built_client,
            roblox_token: self.roblox_token,
            csrf_token: None
        })
    }

    pub fn with_token(mut self, token: &str) -> Self {
        // All .ROBLOSECURITY cookies should start with this
        // unless Roblox just decides to randomly change it some day
        // ...which they might
        if !token.starts_with("_|WARNING") {
            self.error = Some(ClientError::InvalidToken);
            return self;
        }

        let formatted_token = format!(".ROBLOSECURITY={}", token);
        self.roblox_token = Some(formatted_token);
        self
    }
}
