use crate::parser::parser_config::ParserConfig;

pub fn substitute_variables_in_string(
    config: &ParserConfig,
    source: &str,
) -> String {
    let mut new_src = source.to_string();

    for (key, val) in config.variables.iter() {
        new_src = new_src.replace(&format!("${}", key), val);
        new_src = new_src.replace(&format!("$({})", key), val);
    }

    new_src
}