use reqwest::Method;
use crate::client::{RequestComponents, RustbloxClient};
use crate::error::RequestError;
use crate::structs::group::{GroupRolesList, UserGroupList};

const BASE_URL: &str = "https://groups.roblox.com";

impl RustbloxClient {
    /// Gets a list of the group's roles.
    ///
    /// # Errors
    ///
    /// This function will error if:
    /// - The endpoint responds with an error.
    pub async fn get_group_roles(&mut self, group_id: usize) -> Result<GroupRolesList, RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}/roles");

        let components = RequestComponents {
            needs_auth: false,
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
}
