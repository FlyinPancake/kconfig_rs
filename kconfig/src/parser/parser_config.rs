use std::collections::HashMap;

pub struct ParserConfig {
    pub root_path: String,
    pub can_source: bool,
    pub variables: HashMap<String, String>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            root_path: "".to_string(),
            can_source: false,
            variables: HashMap::new(),
        }
    }
}
