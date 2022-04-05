
pub fn strip_quotes(source: &str) -> String {
    let mut src = source.to_string();
    if source.starts_with("\"") {
        src = src[1..].to_string();
    }

    if src.ends_with("\"") {
        src = src[..src.len()-1].to_string();
    }

    src
}