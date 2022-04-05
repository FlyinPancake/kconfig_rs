use std::path::Path;
use crate::errors::parser_error::ParserError;
use crate::parser::utils::get_string_from_path::get_string_from_path;

pub fn read_file_to_string(path: &Path) -> Result<String, ParserError> {
    std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileRead(get_string_from_path(path)))
}