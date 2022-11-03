use crate::error::{ClientError, RequestError};
use log::{warn, debug};

pub struct RustbloxClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) roblox_token: Option<String>,
    pub(crate) csrf_token: Option<String>,
}

impl RustbloxClient {
    pub async fn login(&mut self) -> Result<(), ClientError> {
        // Initial connection test can come first
        // and return early if no cookie is set
        if self.reqwest_client.get("https://roblox.com").send().await.is_err() {
            return Err(ClientError::LoginFailed("Could not ping roblox.com".to_string()));
        }
        debug!("Successfully pinged Roblox");

        if self.roblox_token.is_none() {
            warn!(
                "[WARN/LOGIN]: Please be advised that you do not have a .ROBLOSECURITY cookie set.\
                \nYou will not be able to use functions that require this."
            );
            return Ok(()); // Return early
        }

        let auth_attempt = self.reqwest_client
            .post("https://auth.roblox.com/v2/logout")
            .header("Content-Length", 0)
            .header("Cookie", self.roblox_token.as_ref().unwrap())
            .send()
            .await;
        if let Err(why) = auth_attempt {
            let message = format!("Authentication request errored:\n{}", why);
            return Err(ClientError::LoginFailed(message));
        }

        let auth_response = auth_attempt.unwrap();
        if auth_response.status() != 403 {
            return Err(ClientError::InvalidToken);
        }
        if !auth_response.headers().contains_key("x-csrf-token") {
            return Err(ClientError::LoginFailed("No x-csrf-token was given by Roblox".to_string()));
        }
        let csrf_from_headers = auth_response
            .headers()
            .get("x-csrf-token")
            .unwrap()
            .to_owned();
        let try_csrf_str = csrf_from_headers.to_str();
        if let Err(why) = try_csrf_str {
            let message = format!("Had error parsing the CSRF token as a str:\n{}", why);
            return Err(ClientError::LoginFailed(message));
        }
        self.csrf_token = Some(try_csrf_str.unwrap().to_string());

        Ok(())
    }

    pub fn is_authenticated(&self) -> bool {
        self.roblox_token != None && self.csrf_token != None
    }

    pub(crate) async fn make_request(&self, url: String, method: reqwest::Method, needs_auth: bool) -> Result<reqwest::Response, RequestError> {
        if needs_auth && self.roblox_token == None {
            return Err(RequestError::NotAuthenticated);
        }

        let mut request = self.reqwest_client.request(method, &url);
        if needs_auth {
            request = request
                .header("Cookie", self.roblox_token.as_ref().unwrap())
                .header("x-csrf-token", self.csrf_token.as_ref().unwrap());
        }

        let try_response = request.send().await;
        if let Err(why) = try_response {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let response = try_response.unwrap();
        Ok(response)
    }
}
