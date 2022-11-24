use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use crate::client::{RequestComponents, RustbloxClient};
use crate::error::{RequestError, RobloxApiError, RobloxApiErrors};
use crate::structs::group::{GroupRole, GroupRolesList, JoinRequest, JoinRequestPage, UserGroupList};

const BASE_URL: &str = "https://groups.roblox.com";

impl RustbloxClient {
    /// **MUST AUTHENTICATE**
    ///
    /// Accepts a join request that a given `user_id` sent to a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn accept_user_join_request(&mut self, group_id: usize, user_id: usize) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests/users/{user_id}");

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(0));

        let components = RequestComponents {
            needs_auth: true,
            method: Method::POST,
            url,
            headers: Some(headers),
            body: None
        };

        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Batch accepts a page of join requests sent to a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn batch_accept_requests(&mut self, group_id: usize) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::POST,
            url,
            headers: None,
            body: None
        };
        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Batch denies a page of join requests sent to a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn batch_deny_requests(&mut self, group_id: usize) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::DELETE,
            url,
            headers: None,
            body: None
        };
        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Retrieves a page of join requests sent to a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn batch_get_requests(&mut self, group_id: usize) -> Result<Option<JoinRequestPage>, RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None
        };
        let join_request_data = self
            .make_request::<Option<JoinRequestPage>>(components, false)
            .await?;

        Ok(join_request_data)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Denies a join request that a given `user_id` sent to a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn deny_user_join_request(&mut self, group_id: usize, user_id: usize) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests/users/{user_id}");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::DELETE,
            url,
            headers: None,
            body: None
        };

        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets a list of the group's roles.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn get_group_roles(&mut self, group_id: usize) -> Result<GroupRolesList, RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/roles");
        
        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };
        let ranks = self
            .make_request::<GroupRolesList>(components, false)
            .await?;
        Ok(ranks)
    }

    /// This doesn't need authentication and will be moved at a later date.
    /// TODO: Move this to unauthenticated group endpoints
    ///
    /// Gets a list of groups that a given `user_id` is in, along with their role in each group.
    ///
    /// This function will error if:
    /// - The endpoint responds with an error.
    pub async fn get_user_group_roles(&mut self, user_id: usize) -> Result<UserGroupList, RequestError> {
        let url = format!("{BASE_URL}/v1/users/{user_id}/groups/roles");

        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };
        let info = self
            .make_request::<UserGroupList>(components, false)
            .await?;

        Ok(info)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Gets a join request that a given `user_id` sent to a given `group_id`.
    /// *Will be `None` if the join request doesn't exist.*
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn get_user_join_request(&mut self, group_id: usize, user_id: usize) -> Result<Option<JoinRequest>, RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/join-requests/users/{user_id}");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::GET,
            url,
            headers: None,
            body: None
        };
        let join_request = self
            .make_request::<Option<JoinRequest>>(components, false)
            .await?;

        Ok(join_request)
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Kicks (exiles) a given `user_id` from a given `group_id`.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    pub async fn kick_user(&mut self, group_id: usize, user_id: usize) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/users/{user_id}");

        let components = RequestComponents {
            needs_auth: true,
            method: Method::DELETE,
            url,
            headers: None,
            body: None
        };

        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }

    /// **MUST AUTHENTICATE**
    ///
    /// Sets the role of a given `user_id` in a given `group_id`.
    /// TODO: Figure out how to use GroupRole instead of `role_rank_id`
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - You do not have a `.ROBLOSECURITY` cookie set.
    /// - The endpoint responds with an error.
    /// - No such role exists in the group.
    pub async fn set_user_role_in_group(&mut self, group_id: usize, user_id: usize, role_rank_id: u8) -> Result<(), RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/users/{user_id}");

        let roles = self.get_group_roles(group_id).await?;
        let desired_role: Vec<GroupRole> = roles.roles.into_iter().filter(|role| role.rank == role_rank_id).collect();
        if desired_role.is_empty() {
            let error = RobloxApiErrors {
                errors: vec![
                    RobloxApiError {
                        code: 2,
                        message: "The roleset is invalid or does not exist.".to_string(),
                    }
                ],
            };
            return Err(RequestError::ClientError(url, 400, error));
        }

        let data_json = json!({
            "roleId": desired_role.first().unwrap().id
        });
        let data_size = data_json.clone().to_string().as_bytes().len();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Length", HeaderValue::from(data_size));
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );

        let components = RequestComponents {
            needs_auth: true,
            method: Method::PATCH,
            url,
            headers: Some(headers),
            body: Some(data_json.to_string()),
        };

        self
            .make_request::<serde_json::Value>(components, false)
            .await?;

        Ok(())
    }
}
