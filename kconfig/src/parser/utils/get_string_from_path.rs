use std::path::Path;

pub fn get_string_from_path(path: &Path) -> String {
    path.to_str().unwrap_or("<unknown>").to_string()
}