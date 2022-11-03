use crate::client::RustbloxClient;
use crate::error::RequestError;
use crate::structs::user::UserInfo;
use reqwest::Method;

const BASE_URL: &str = "https://users.roblox.com/v1";

impl RustbloxClient {
    pub async fn get_user_info(&self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{}/users/{}", BASE_URL, id);
        let response = self
            .make_request(url.clone(), Method::GET, false)
            .await?;
        let try_user_info = response
            .json::<UserInfo>()
            .await;
        if try_user_info.is_err() {
            return Err(RequestError::RequestError(url, "Had error parsing data".to_string()));
        }
        Ok(try_user_info.unwrap())
    }
}
