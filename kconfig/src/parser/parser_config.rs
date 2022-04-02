pub struct ParserConfig {
    pub can_source: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            can_source: false,
        }
    }
}
