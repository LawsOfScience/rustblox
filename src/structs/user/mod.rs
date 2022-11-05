#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    description: String,
    created: String,
    isBanned: bool,
    externalAppDisplayName: Option<String>,
    hasVerifiedBadge: bool,
    id: usize,
    name: String,
    displayName: String
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfo {
    displayName: String,
    hasVerifiedBadge: bool,
    id: usize,
    name: String
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithRequestedName {
    requestedUsername: String,
    displayName: String,
    hasVerifiedBadge: bool,
    id: usize,
    name: String
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithPreviousNames {
    previousUsernames: Option<Vec<String>>,
    displayName: String,
    hasVerifiedBadge: bool,
    id: usize,
    name: String
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserSearchPage {
    previousPageCursor: Option<String>,
    nextPageCursor: Option<String>,
    data: Vec<MinimalUserInfoWithPreviousNames>
}
