use crate::structs::user::MinimalUserInfo;

/// Represents a join request to a group. Used in
/// [`get_user_join_request`](crate::client::RustbloxClient::get_user_join_request)
/// as well as in [`JoinRequestPage`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct JoinRequest {
    pub requester: MinimalUserInfo,
    pub created: String,
}

/// Represents a page of join requests to a group. Used in
/// [`batch_get_requests`](crate::client::RustbloxClient::batch_get_requests).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JoinRequestPage {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<JoinRequest>,
}

/// Represents a role in a group. Used as a component
/// of [`GroupRolesList`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub rank: u8,
    pub member_count: Option<usize>,
}

/// Represents a list of roles in a group. Used in
/// [`get_group_roles`](crate::client::RustbloxClient::get_group_roles).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupRolesList {
    pub group_id: usize,
    pub roles: Vec<GroupRole>,
}

/// Represents a list of groups that a user is in. Used
/// in [`get_user_group_roles`](crate::client::RustbloxClient::get_user_group_roles).
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserGroupList {
    pub data: Vec<UserGroup>
}

/// Represents a group that a user is in, containing the group info
/// and info about the user's rank/role in it. Used as a component
/// of [`UserGroupList`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserGroup {
    pub group: UserGroupInfo,
    pub role: UserRoleInGroup,
    #[serde(default)]
    pub is_primary_group: bool,
}

/// Contains information about a group's shout, if there is one.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupShout {
    pub body: String,
    pub poster: MinimalUserInfo,
}

/// Contains all the information about a group that a certain user
/// is in. Used as a component of [`UserGroup`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupInfo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub owner: MinimalUserInfo,
    pub shout: Option<GroupShout>,
    pub member_count: usize,
    pub is_builders_club_only: bool,
    pub public_entry_allowed: bool,
    pub has_verified_badge: bool,
}

/// Contains information about a user's role in a certain group. Used
/// as a component of [`UserGroup`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserRoleInGroup {
    pub id: usize,
    pub name: String,
    pub rank: u8,
}

/// Represents a page of users in a group's role. Used in
/// [`get_group_role_members`](crate::client::RustbloxClient::get_group_role_members)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoleMembersPage {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<MinimalUserInfo>,
}

/// Contains information about a member of a group. Used as a component
/// of [`GroupMembersPage`](GroupMembersPage)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GroupMemberInfo {
    pub user: MinimalUserInfo,
    pub role: GroupRole,
}

/// Represents a page of information about the members of a group.
/// Used in [`get_group_members`](crate::client::RustbloxClient::get_group_members)
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembersPage {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<GroupMemberInfo>,
}
