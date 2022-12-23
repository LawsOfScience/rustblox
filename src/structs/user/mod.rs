#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub description: String,
    pub created: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "externalAppDisplayName")]
    pub external_app_display_name: Option<String>,
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct MinimalUserInfo {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(alias = "userId")]
    pub id: usize,
    #[serde(alias = "username")]
    pub name: String,
}
// I love the #[serde(alias)] macro since, for some reason, Roblox
// decided to change the names of certain JSON objects
// DESPITE THEM LITERALLY HAVING THE SAME DATA.

#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithRequestedName {
    #[serde(rename = "requestedUsername")]
    pub requested_username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithPreviousNames {
    #[serde(rename = "previousUsernames")]
    pub previous_usernames: Option<Vec<String>>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UserSearchPage {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<MinimalUserInfoWithPreviousNames>,
}
