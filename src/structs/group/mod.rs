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
