#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub description: String,
    pub created: String,
    pub isBanned: bool,
    pub externalAppDisplayName: Option<String>,
    pub hasVerifiedBadge: bool,
    pub id: usize,
    pub name: String,
    pub displayName: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfo {
    pub displayName: String,
    pub hasVerifiedBadge: bool,
    pub id: usize,
    pub name: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithRequestedName {
    pub requestedUsername: String,
    pub displayName: String,
    pub hasVerifiedBadge: bool,
    pub id: usize,
    pub name: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MinimalUserInfoWithPreviousNames {
    pub previousUsernames: Option<Vec<String>>,
    pub displayName: String,
    pub hasVerifiedBadge: bool,
    pub id: usize,
    pub name: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UserSearchPage {
    pub previousPageCursor: Option<String>,
    pub nextPageCursor: Option<String>,
    pub data: Vec<MinimalUserInfoWithPreviousNames>,
}
