pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Deserialize, Debug)]
pub struct RobloxAPIError {
    pub code: isize,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct RobloxAPIErrors {
    pub errors: Vec<RobloxAPIError>
}
