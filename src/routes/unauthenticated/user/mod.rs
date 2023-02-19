use crate::client::RequestComponents;
use crate::client::RustbloxClient;
use crate::error::RequestError;
use crate::structs::user::{MinimalUserInfo, MinimalUserInfoWithRequestedName, PreviousUsernamesPage, UserInfo, UserSearchPage};
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

impl RustbloxClient {
    /// Gets a user's previous usernames, given their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 3: The user ID is invalid
    pub async fn get_previous_usernames(
        &mut self,
        id: usize,
        limit: Option<usize>,
        cursor: Option<String>,
        sort_order: Option<SortOrder>,
    ) -> Result<PreviousUsernamesPage, RequestError> {
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
        Ok(previous_usernames_data)
    }

    /// Gets the info about a user from their user ID.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 404 code 3: The user ID is invalid
    pub async fn get_user_info(&mut self, id: usize) -> Result<UserInfo, RequestError> {
        let url = format!("{BASE_URL}/users/{id}");
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

    /// Searches for users by their IDs.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 1: Too many IDs
    pub async fn get_users_from_ids(
        &mut self,
        ids: Vec<usize>,
        exclude_banned: bool,
    ) -> Result<Vec<MinimalUserInfo>, RequestError> {
        let url = format!("{BASE_URL}/users");
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

        Ok(response.data)
    }

    /// Searches for users by their usernames.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 2: Too many usernames
    pub async fn get_users_from_usernames(
        &mut self,
        usernames: Vec<&str>,
        exclude_banned: bool,
    ) -> Result<Vec<MinimalUserInfoWithRequestedName>, RequestError> {
        let url = format!("{BASE_URL}/usernames/users");
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

        Ok(response.data)
    }

    /// Searches for a specific user.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made, or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 5: The username (keyword) was filtered
    /// - Status 400 code 6: The username (keyword) is too short
    /// - Status 429 code 4: Too many requests
    pub async fn search_user(
        &mut self,
        username: String,
        limit: Option<usize>,
        page_cursor: Option<String>,
    ) -> Result<UserSearchPage, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url = format!(
            "{BASE_URL}/users/search?keyword={username}&limit={real_limit}"
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
