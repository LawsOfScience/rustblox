use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use crate::client::{RequestComponents, RustbloxClient};
use crate::error::RequestError;
use crate::structs::group::{JoinRequest, JoinRequestPage};

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
}
