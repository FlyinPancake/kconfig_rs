
pub struct ParseSpan<'a, 's> {
    pub source_span: &'a [&'s str],
}

pub fn find_index_of_next_keyword(keyword: &str, span: ParseSpan) -> Option<usize> {
    span.source_span
        .iter()
        .position(|el| *el == keyword)
}
