// Why doesn't the Roblox API return a common user data JSON object???
#[derive(Deserialize, Debug)]
pub struct JoinRequesterInfo {
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(rename = "userId")]
    pub user_id: usize,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct JoinRequest {
    pub requester: JoinRequesterInfo,
    pub created: String,
}

#[derive(Deserialize, Debug)]
pub struct JoinRequestPage {
    #[serde(rename = "previousPageCursor")]
    pub previous_page_cursor: Option<String>,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub data: Vec<JoinRequest>,
}

#[derive(Deserialize, Debug)]
pub struct GroupRole {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub rank: u8,
    #[serde(rename = "memberCount")]
    pub member_count: usize,
}

#[derive(Deserialize, Debug)]
pub struct GroupRolesList {
    #[serde(rename = "groupId")]
    pub group_id: usize,
    pub roles: Vec<GroupRole>,
}

#[derive(Deserialize, Debug)]
pub struct UserGroupList {
    pub data: Vec<UserGroup>
}

#[derive(Deserialize, Debug)]
pub struct UserGroup {
    pub group: UserGroupInfo,
    pub role: UserRoleInGroup,
}

#[derive(Deserialize, Debug)]
pub struct UserGroupInfo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub owner: UserGroupOwnerInfo,
    pub shout: Option<String>,
    #[serde(rename = "memberCount")]
    pub member_count: usize,
    #[serde(rename = "isBuildersClubOnly")]
    pub is_builders_club_only: bool,
    #[serde(rename = "publicEntryAllowed")]
    pub public_entry_allowed: bool,
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(default)]
    #[serde(rename = "isPrimaryGroup")]
    pub is_primary_group: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserGroupOwnerInfo {
    #[serde(rename = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(rename = "userId")]
    pub user_id: usize, // ROBLOX, WHY DOES MinimalUserInfo HAVE "id" BUT THIS HAS "userId"?!?!?!?!
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct UserRoleInGroup {
    pub id: usize,
    pub name: String,
    pub rank: u8,
}
