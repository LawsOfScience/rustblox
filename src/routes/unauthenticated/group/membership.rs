use crate::client::{RequestComponents, RustbloxClient};
use crate::error::RequestError;
use crate::structs::group::{GroupMemberInfo, GroupRolesList, UserGroupList};
use crate::structs::{Page, SortOrder};
use reqwest::Method;
use crate::structs::user::MinimalUserInfo;

const BASE_URL: &str = "https://groups.roblox.com";

impl RustbloxClient {
    /// Gets a list of the group's members.
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made,
    /// or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 1: The group is invalid or doesn't exist
    pub async fn get_group_members(
        &mut self,
        group_id: usize,
        limit: Option<usize>,
        cursor: Option<String>,
        sort_order: Option<SortOrder>,
    ) -> Result<Page<GroupMemberInfo>, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url = format!("{BASE_URL}/v1/groups/{group_id}/users?limit={real_limit}");
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
            url,
            headers: None,
            body: None,
        };
        let users = self
            .make_request::<Page<GroupMemberInfo>>(components, false)
            .await?;

        Ok(users)
    }

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

    /// Gets the members in a certain role of a certain group.
    ///
    /// Note that role_id is *not* the same as the position (i.e 255).
    /// The role IDs can be obtained through
    /// [`get_group_roles`](RustbloxClient::get_group_roles).
    ///
    /// # Errors
    ///
    /// This function returns an error if the request could not be made,
    /// or if the endpoint responded with an error.
    ///
    /// Possible error responses:
    /// - Status 400 code 1: The group is invalid or doesn't exist
    /// - Status 403 code 2: The role is invalid or doesn't exist
    pub async fn get_group_role_members(
        &mut self,
        group_id: usize,
        role_id: usize,
        limit: Option<usize>,
        cursor: Option<String>,
        sort_order: Option<SortOrder>
    ) -> Result<Page<MinimalUserInfo>, RequestError> {
        let real_limit = if limit.is_some() { limit.unwrap() } else { 10 };
        let mut url =
            format!("{BASE_URL}/v1/groups/{group_id}/roles/{role_id}/users?limit={real_limit}");
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

        let role_members_data = self
            .make_request::<Page<MinimalUserInfo>>(components, false)
            .await?;
        Ok(role_members_data)
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
