use std::path::Path;
use crate::errors::parser_error::ParserError;

pub fn read_file_to_string(path: &Path) -> Result<String, ParserError> {
    std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileRead(path.to_str().unwrap_or("").to_string()))
}