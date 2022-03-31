use crate::parser::kconfig_parser_impl::kconfig_help_property_parser::parse_and_span_kconfig_help_property;
use crate::parser::utils::rdp::ParseSpan;

#[test]
fn happy_path_help_parsing() {
    let source = "\thelp\n\
        \t\tendmenu lol\n\
        \t\tkeke sajt\n\
        \tdepline\n\
        \tdepline\n\
        ";
    let lines_iter = source.lines().collect::<Vec<&str>>();
    let span = ParseSpan::new(&lines_iter[..]);

    let parse_res = parse_and_span_kconfig_help_property(span).unwrap();
    assert_eq!(parse_res.1, 2);
    assert_eq!(parse_res.0.help_text, "endmenu lol\nkeke sajt")
}
