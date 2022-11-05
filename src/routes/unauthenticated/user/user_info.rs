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
    pub async fn get_user_info(&self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{}/users/{}", BASE_URL, id);
        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None
        };

        let response = self
            .make_request(components)
            .await?;
        let try_user_info = response
            .json::<UserInfo>()
            .await;
        if try_user_info.is_err() {
            return Err(RequestError::RequestError(url, "Had error parsing data".to_string()));
        }
        Ok(try_user_info.unwrap())
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

        let try_response = self
            .make_request(components)
            .await;
        if let Err(why) = try_response {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let try_body_json = try_response
            .unwrap()
            .json::<MinimalUserInfoObject>()
            .await;
        if let Err(why) = try_body_json {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let body = try_body_json.unwrap();
        let mut user_info_vec: Vec<MinimalUserInfo> = Vec::new();
        for minimal_user in body.data {
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

        let try_response = self
            .make_request(components)
            .await;
        if let Err(why) = try_response {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let try_body_json = try_response
            .unwrap()
            .json::<MinimalUserInfoWithReqdObject>()
            .await;
        if let Err(why) = try_body_json {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let body = try_body_json.unwrap();
        let mut user_info_vec: Vec<MinimalUserInfoWithRequestedName> = Vec::new();
        for minimal_user in body.data {
            user_info_vec.push(minimal_user);
        }

        Ok(user_info_vec)
    }

    pub async fn search_users(&self, username: String, limit: Option<usize>, page_cursor: Option<String>) -> Result<UserSearchPage, RequestError> {
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

        let try_response = self
            .make_request(components)
            .await;
        if let Err(why) = try_response {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let try_body_json = try_response
            .unwrap()
            .json::<UserSearchPage>()
            .await;
        if let Err(why) = try_body_json {
            return Err(RequestError::RequestError(url, why.to_string()));
        }
        let body = try_body_json.unwrap();

        Ok(body)
    }
}
