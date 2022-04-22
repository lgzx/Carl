pub enum SearchType {
    REGEX(String),
    STRING(String),
}
pub struct SearchPattern {
    pub pattern: SearchType,
}
