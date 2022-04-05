pub fn collapse_manual_line_breaks(
    lines: Vec<&str>
) -> Vec<String> {
    let mut next_line_collapsed = false;
    let mut out_vec: Vec<String> = vec![];
    for line in lines {
        if next_line_collapsed {
            let last_ind = out_vec.len() - 1;
            out_vec[last_ind] += line.trim();

            next_line_collapsed = false;
            continue;
        }

        if line.ends_with("\\") {
            next_line_collapsed = true;
            out_vec.push(line[..line.len()-1].to_string());
            continue;
        }

        out_vec.push(line.to_string());
    }

    out_vec
}

#[cfg(test)]
mod test {
    use crate::parser::utils::collapse_manual_line_breaks::collapse_manual_line_breaks;

    #[test]
    fn collapses_lines() {
        let source = "config SUSPEND_FREEZER\n\
        \tbool \"Enable freezer for suspend to RAM/standby\" \\\n\
        \t\tkeke sajt\n\
        endmenu\n\
        ";
        let out = "config SUSPEND_FREEZER\n\
        \tbool \"Enable freezer for suspend to RAM/standby\" keke sajt\n\
        endmenu\n\
        ";

        assert_eq!(
            collapse_manual_line_breaks(source.lines().collect::<Vec<&str>>()),
            out.lines().map(|el| el.to_string()).collect::<Vec<String>>(),
        )
    }
}
