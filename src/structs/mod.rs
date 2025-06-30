pub mod user;
pub mod group;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DataWrapper<T> {
    pub data: Vec<T>,
}
