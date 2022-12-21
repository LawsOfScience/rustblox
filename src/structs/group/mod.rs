// Why doesn't the Roblox API return a common user data JSON object???
#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct JoinRequesterInfo {
    pub hasVerifiedBadge: bool,
    pub userId: usize,
    pub username: String,
    pub displayName: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JoinRequest {
    pub requester: JoinRequesterInfo,
    pub created: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct JoinRequestPage {
    pub previousPageCursor: Option<String>,
    pub nextPageCursor: Option<String>,
    pub data: Vec<JoinRequest>,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct GroupRole {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub rank: u8,
    pub memberCount: usize,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct GroupRolesList {
    pub groupId: usize,
    pub roles: Vec<GroupRole>,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserGroupList {
    pub data: Vec<UserGroup>
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserGroup {
    pub group: UserGroupInfo,
    pub role: UserRoleInGroup,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserGroupInfo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub owner: UserGroupOwnerInfo,
    pub shout: Option<String>,
    pub memberCount: usize,
    pub isBuildersClubOnly: bool,
    pub publicEntryAllowed: bool,
    pub hasVerifiedBadge: bool,
    #[serde(default)]
    pub isPrimaryGroup: bool,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserGroupOwnerInfo {
    pub hasVerifiedBadge: bool,
    pub userId: usize, // ROBLOX, WHY DOES MinimalUserInfo HAVE "id" BUT THIS HAS "userId"?!?!?!?!
    pub username: String,
    pub displayName: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserRoleInGroup {
    pub id: usize,
    pub name: String,
    pub rank: u8,
}
