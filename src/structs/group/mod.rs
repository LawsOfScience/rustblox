// Why doesn't the Roblox API return a common user data JSON object???
#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct JoinRequesterInfo {
    hasVerifiedBadge: bool,
    userId: usize,
    username: String,
    displayName: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct JoinRequest {
    requester: JoinRequesterInfo,
    created: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct JoinRequestPage {
    previousPageCursor: Option<String>,
    nextPageCursor: Option<String>,
    data: Vec<JoinRequest>,
}
