pub(crate) fn get_line_indent(line: &str) -> usize {
    let mut indent = 0;

    for chr in line.chars() {
        if !chr.is_whitespace() {
            break;
        }

        if chr == '\t' {
            indent += 8;
        } else {
            indent += 1;
        }
    }

    indent
}

#[cfg(test)]
mod test {
    use crate::parser::utils::get_line_indent::get_line_indent;

    #[test]
    fn basic_indent_count_works_one() {
        assert_eq!(
            get_line_indent("\t\t\thello world lmao."),
            24
        );
    }

    #[test]
    fn basic_indent_count_works_two() {
        assert_eq!(
            get_line_indent("  \t\thello world \t lmao."),
            18
        );
    }
}