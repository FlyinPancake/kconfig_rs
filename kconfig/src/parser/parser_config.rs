use std::collections::HashMap;

pub struct ParserConfig {
    pub can_source: bool,
    pub variables: HashMap<String, String>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            can_source: false,
            variables: HashMap::new(),
        }
    }
}
