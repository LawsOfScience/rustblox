pub mod group;
pub mod user;

/// Represents a sorting order for API endpoints
/// that can use it.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// Represents a page of data for paginated endpoints.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<T>,
}

/// Convenience struct used for endpoints that return
/// a response of the form
/// ```json
/// {
///   "data": [ ... ]
/// }
/// ```
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataWrapper<T> {
    pub data: Vec<T>,
}
