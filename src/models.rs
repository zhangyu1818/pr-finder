#[derive(Debug)]
pub struct SearchResult {
    pub number: i64,
    pub title: String,
    pub date: chrono::DateTime<chrono::Local>,
}
