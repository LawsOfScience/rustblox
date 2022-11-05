use crate::client::RustbloxClient;
use crate::error::RequestError;
use crate::structs::user::UserInfo;
use reqwest::Method;

const BASE_URL: &str = "https://users.roblox.com/v1";

impl RustbloxClient {
    /// Gets the info about a user from their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    pub async fn get_user_info(&self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{}/users/{}", BASE_URL, id);

        let response = self.make_request(url.clone(), Method::GET, false).await?;

        let user_info = response
            .json::<UserInfo>()
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        Ok(user_info)
    }
}
