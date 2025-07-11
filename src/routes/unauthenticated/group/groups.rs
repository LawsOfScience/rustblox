use reqwest::Method;

use crate::{client::{RequestComponents, RustbloxClient}, error::RequestError, structs::group::GroupInfo};

const BASE_URL: &str = "https://groups.roblox.com";

impl RustbloxClient {
    /// Gets information about a given `group_id`.
    ///
    /// This function will error if:
    /// - The endpoint reqponds with an error.
    pub async fn get_group_info(&self, group_id: usize) -> Result<GroupInfo, RequestError> {
        let url = format!("{BASE_URL}/v1/groups/{group_id}");

        let components = RequestComponents {
            needs_auth: false,
            method: Method::GET,
            url,
            headers: None,
            body: None,
        };
        let info = self
            .make_request::<GroupInfo>(components, false)
            .await?;

        Ok(info)
    }
}
