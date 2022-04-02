use crate::errors::parser_error::ParserError;

pub struct ParseSpan<'a, 's, 'f> {
    filename: &'f str,
    global_span: (usize, usize),
    source_span: &'a [&'s str],
}

pub struct LineSpan<'s, 'f> {
    filename: &'f str,
    line: &'s str,
    global_at: usize,
}

impl<'s, 'f> LineSpan<'s, 'f> {
    pub fn get_line(&self) -> &'s str {
        self.line
    }

    pub fn get_filename(&self) -> &'f str {
        self.filename
    }

    pub fn get_global_at(&self) -> usize {
        self.global_at
    }
}

impl<'a, 's, 'f> ParseSpan<'a, 's, 'f> {
    pub fn from_source(source_span: &'a [&'s str], filename: &'f str) -> Self {
        Self {
            filename,
            source_span,
            global_span: (0, source_span.len() - 1),
        }
    }

    pub fn non_empty_or(&self) -> Result<(), ParserError> {
        if self.source_span.len() == 0 {
            Err(ParserError::Internal(
                "Expected a non empty span.".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.source_span.len()
    }

    pub fn get_with_start_at(&self, at: usize) -> Self {
        Self {
            filename: self.filename,
            source_span: &self.source_span[at..],
            global_span: (self.global_span.0 + at, self.global_span.1),
        }
    }

    pub fn get_with_end_at(&self, at: usize) -> Self {
        Self {
            filename: self.filename,
            source_span: &self.source_span[..=at],
            global_span: (self.global_span.0, self.global_span.0 + at),
        }
    }

    pub fn get_with_bounds(&self, from: usize, to: usize) -> Self {
        Self {
            filename: self.filename,
            source_span: &self.source_span[from..=to],
            global_span: (self.global_span.0 + from, self.global_span.0 + to),
        }
    }

    pub fn get_source_span(&self) -> &'a [&'s str] {
        self.source_span
    }

    pub fn get_filename(&self) -> &'f str {
        self.filename
    }

    pub fn get_global_span(&self) -> (usize, usize) {
        self.global_span
    }

    pub fn get_line_span_at(&self, offset: usize) -> LineSpan<'s, 'f> {
        LineSpan {
            filename: self.filename,
            line: self.source_span[offset],
            global_at: self.global_span.0 + offset,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::utils::parse_span::ParseSpan;

    #[test]
    fn happy_path_offseting_works() {
        let source_code = "hello\n\
        world\n\
        you\n\
        people";
        let lines = source_code.lines().collect::<Vec<&str>>();
        let full_span = ParseSpan::from_source(&lines[..], "test.kconfig");

        assert_eq!(full_span.get_global_span(), (0, 3));
        assert_eq!(full_span.get_filename(), "test.kconfig");

        let hello_world = full_span.get_with_end_at(1);
        assert_eq!(hello_world.get_global_span(), (0, 1));
        assert_eq!(hello_world.get_source_span()[1], "world");
        assert_eq!(hello_world.get_source_span()[0], "hello");

        let you = full_span.get_with_bounds(2, 2);
        assert_eq!(you.get_global_span(), (2, 2));
        assert_eq!(you.get_source_span()[0], "you");
    }
}
