use crate::client::RequestComponents;
use crate::client::RustbloxClient;
use crate::error::RequestError;
use crate::structs::user::{
    MinimalUserInfo, MinimalUserInfoWithRequestedName, UserInfo, UserSearchPage,
};
use crate::structs::SortOrder;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;

const BASE_URL: &str = "https://users.roblox.com/v1";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MinimalUserInfoObject {
    data: Vec<MinimalUserInfo>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MinimalUserInfoWithReqdObject {
    data: Vec<MinimalUserInfoWithRequestedName>,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
struct PreviousUsername {
    name: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
struct PreviousUsernamesPage {
    previousPageCursor: Option<String>,
    nextPageCursor: Option<String>,
    data: Vec<PreviousUsername>,
}

impl RustbloxClient {
    /// Gets a user's previous usernames, given their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    pub async fn get_previous_usernames(
        &mut self,
        id: usize,
        limit: Option<usize>,
        cursor: Option<String>,
        sort_order: Option<SortOrder>,
    ) -> Result<Vec<String>, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url = format!("{BASE_URL}/users/{id}/username-history?limit={real_limit}");
        if cursor.is_some() {
            url = format!("{url}&cursor={}", cursor.unwrap());
        }
        if sort_order.is_some() {
            match sort_order.unwrap() {
                SortOrder::Ascending => url = format!("{url}&sortOrder=Asc"),
                SortOrder::Descending => url = format!("{url}&sortOrder=Desc"),
            }
        }

        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None,
        };

        let previous_usernames_data = self
            .make_request::<PreviousUsernamesPage>(components, false)
            .await?;
        let mut previous_usernames: Vec<String> = Vec::new();
        for username in previous_usernames_data.data {
            previous_usernames.push(username.name)
        }

        Ok(previous_usernames)
    }
    /// Gets the info about a user from their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    pub async fn get_user_info(&mut self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{}/users/{}", BASE_URL, id);
        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None,
        };

        let user_info = self.make_request::<UserInfo>(components, false).await?;

        Ok(user_info)
    }

    pub async fn get_users_from_ids(
        &mut self,
        ids: Vec<usize>,
        exclude_banned: bool,
    ) -> Result<Vec<MinimalUserInfo>, RequestError> {
        let url = format!("{}/users", BASE_URL);
        let data_json = json!({
            "userIds": ids,
            "excludeBannedUsers": exclude_banned
        });
        let data_size = data_json.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let components = RequestComponents {
            needs_auth: false,
            method: Method::POST,
            url: url.clone(),
            headers: Some(headers),
            body: Some(data_json.to_string()),
        };

        let response = self
            .make_request::<MinimalUserInfoObject>(components, false)
            .await?;

        let mut user_info_vec: Vec<MinimalUserInfo> = Vec::new();
        for minimal_user in response.data {
            user_info_vec.push(minimal_user);
        }

        Ok(user_info_vec)
    }

    pub async fn get_users_from_usernames(
        &mut self,
        usernames: Vec<&str>,
        exclude_banned: bool,
    ) -> Result<Vec<MinimalUserInfoWithRequestedName>, RequestError> {
        let url = format!("{}/usernames/users", BASE_URL);
        let data_json = json!({
            "usernames": usernames,
            "excludeBannedUsers": exclude_banned
        });
        let data_size = data_json.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let components = RequestComponents {
            needs_auth: false,
            method: Method::POST,
            url: url.clone(),
            headers: Some(headers),
            body: Some(data_json.to_string()),
        };

        let response = self
            .make_request::<MinimalUserInfoWithReqdObject>(components, false)
            .await?;

        let mut user_info_vec: Vec<MinimalUserInfoWithRequestedName> = Vec::new();
        for minimal_user in response.data {
            user_info_vec.push(minimal_user);
        }

        Ok(user_info_vec)
    }

    pub async fn search_user(
        &mut self,
        username: String,
        limit: Option<usize>,
        page_cursor: Option<String>,
    ) -> Result<UserSearchPage, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url = format!(
            "{}/users/search?keyword={}&limit={}",
            BASE_URL, username, real_limit
        );
        if page_cursor.is_some() {
            url = format!("{}&cursor={}", url, page_cursor.unwrap());
        }

        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url: url.clone(),
            headers: None,
            body: None,
        };

        let response = self
            .make_request::<UserSearchPage>(components, false)
            .await?;

        Ok(response)
    }
}
