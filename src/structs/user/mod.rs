/// Contains the expanded set of user info provided by
/// the Roblox API. Used primarily by
/// [`get_user_info`](crate::client::RustbloxClient::get_user_info).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub description: String,
    pub created: String,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

/// Contains minimal information about the authenticated user.
/// Used in [`get_authenticated_user`](crate::client::RustbloxClient::get_authenticated_user).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinimalAuthenticatedUser {
    pub id: usize,
    pub name: String,
    pub display_name: String,
}

/// Contains information about the authenticated user's age bracket.
/// Used in [`get_authenticated_user_age_bracket`](crate::client::RustbloxClient::get_authenticated_user_age_bracket).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserAgeBracket {
    pub age_bracket: usize
}

/// Contains information about the authenticated user's country code.
/// Used in [`get_authenticated_user_country_code`](crate::client::RustbloxClient::get_authenticated_user_country_code).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserCountryCode {
    pub country_code: String
}

/// Contains information about the authenticated user's country code.
/// Used in [`get_authenticated_user_roles`](crate::client::RustbloxClient::get_authenticated_user_roles).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUserRoles {
    pub roles: Vec<String>
}

/// Contains a minimal set of user info provided by
/// the Roblox API. Used throughout a variety of functions
/// whenever a small amount of user info is returned as part of
/// a larger body.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinimalUserInfo {
    pub display_name: String,
    pub has_verified_badge: bool,
    #[serde(alias = "userId")]
    pub id: usize,
    #[serde(alias = "username")]
    pub name: String,
}
// I love the #[serde(alias)] macro since, for some reason, Roblox
// decided to change the names of certain JSON objects
// DESPITE THEM LITERALLY HAVING THE SAME DATA.

/// Contains a minimal set of user info provided by
/// the Roblox API, along with the username that was requested
/// to search by. Used primarily by
/// [`get_users_from_usernames`](crate::client::RustbloxClient::get_users_from_usernames).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinimalUserInfoWithRequestedName {
    pub requested_username: String,
    pub display_name: String,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
}

/// Contains a minimal set of user info provided by
/// the Roblox API, along with the user's previous usernames.
/// Used primarily in [`search_user`](crate::client::RustbloxClient::search_user).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MinimalUserInfoWithPreviousNames {
    pub previous_usernames: Option<Vec<String>>,
    pub display_name: String,
    pub has_verified_badge: bool,
    pub id: usize,
    pub name: String,
}

/// Represents a user's previous username. Used as a component
/// of [`PreviousUsernamesPage`]. Used primarily in
/// [`get_previous_usernames`](crate::client::RustbloxClient::get_previous_usernames).
#[allow(dead_code, non_snake_case)]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PreviousUsername {
    name: String,
}

