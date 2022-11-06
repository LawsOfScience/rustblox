use reqwest::header::{HeaderMap, HeaderValue};
use crate::client::RustbloxClient;
use crate::error::RequestError;
use crate::structs::user::{UserInfo, MinimalUserInfo, MinimalUserInfoWithRequestedName, UserSearchPage};
use crate::client::RequestComponents;
use reqwest::Method;

const BASE_URL: &str = "https://users.roblox.com/v1";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MinimalUserInfoObject {
    data: Vec<MinimalUserInfo>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MinimalUserInfoWithReqdObject {
    data: Vec<MinimalUserInfoWithRequestedName>
}

impl RustbloxClient {
    /// Gets the info about a user from their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    pub async fn get_user_info(&self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{}/users/{}", BASE_URL, id);
        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None
        };

        /*
        let response = self
            .make_request(components)
            .await?;
        let user_info = response
            .json::<UserInfo>()
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;
        */
        let user_info = self
            .make_request::<UserInfo>(components)
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        Ok(user_info)
    }

    pub async fn get_users_from_ids(&self, ids: Vec<usize>, exclude_banned: bool) -> Result<Vec<MinimalUserInfo>, RequestError> {
        let url = format!("{}/users", BASE_URL);
        let data_json = json!({
            "userIds": ids,
            "excludeBannedUsers": exclude_banned
        });
        let data_size = data_json.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

        let components = RequestComponents {
            needs_auth: false,
            method: Method::POST,
            url: url.clone(),
            headers: Some(headers),
            body: Some(data_json.to_string())
        };

        let response = self
            .make_request::<MinimalUserInfoObject>(components)
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        let mut user_info_vec: Vec<MinimalUserInfo> = Vec::new();
        for minimal_user in response.data {
            user_info_vec.push(minimal_user);
        }

        Ok(user_info_vec)
    }

    pub async fn get_users_from_usernames(&self, usernames: Vec<&str>, exclude_banned: bool) -> Result<Vec<MinimalUserInfoWithRequestedName>, RequestError> {
        let url = format!("{}/usernames/users", BASE_URL);
        let data_json = json!({
            "usernames": usernames,
            "excludeBannedUsers": exclude_banned
        });
        let data_size = data_json.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

        let components = RequestComponents {
            needs_auth: false,
            method: Method::POST,
            url: url.clone(),
            headers: Some(headers),
            body: Some(data_json.to_string())
        };

        let response = self
            .make_request::<MinimalUserInfoWithReqdObject>(components)
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        let mut user_info_vec: Vec<MinimalUserInfoWithRequestedName> = Vec::new();
        for minimal_user in response.data {
            user_info_vec.push(minimal_user);
        }

        Ok(user_info_vec)
    }

    pub async fn search_user(&self, username: String, limit: Option<usize>, page_cursor: Option<String>) -> Result<UserSearchPage, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url = format!("{}/users/search?keyword={}&limit={}", BASE_URL, username, real_limit);
        if page_cursor.is_some() {
            url = format!("{}&cursor={}", url, page_cursor.unwrap());
        }

        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None
        };

        let response = self
            .make_request::<UserSearchPage>(components)
            .await
            .map_err(|e| RequestError::RequestError(url, e.to_string()))?;

        Ok(response)
    }
}
