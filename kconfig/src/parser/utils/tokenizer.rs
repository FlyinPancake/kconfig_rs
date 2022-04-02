use std::str::Chars;

pub struct LineKConfigTokenizerIterator<'s> {
    line: &'s str,

    at_pos: usize,
    at_char: Option<char>,
    last_char: Option<char>,

    chars_iter: Chars<'s>,
}

impl<'s> LineKConfigTokenizerIterator<'s> {
    pub fn from_line(line: &'s str) -> Self {
        let mut inner_iter = line.chars();
        let at_char = inner_iter.next();
        Self {
            line,
            at_pos: 0,
            at_char,
            last_char: None,
            chars_iter: inner_iter,
        }
    }

    fn advance_one(&mut self) {
        self.at_pos += 1;
        self.last_char = self.at_char.clone();
        self.at_char = self.chars_iter.next();
    }

    fn advance_while(&mut self, while_fn: impl Fn(&Self) -> bool) {
        while self.at_char.is_some() && while_fn(self) {
            self.advance_one();

            if self.at_char.is_none() {
                break;
            }
        }
    }

    fn advance_while_is_whitespace_is(&mut self, whitespace: bool) {
        self.advance_while(|iter| {
            if let Some(at_char) = iter.at_char {
                at_char.is_whitespace() == whitespace
            } else {
                false
            }
        });
    }

    pub fn get_remaining_slice(&self) -> &'s str {
        self.line[self.at_pos]
    }
}

impl<'s> Iterator for LineKConfigTokenizerIterator<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_while_is_whitespace_is(true);
        let start_at = self.at_pos;

        if self.at_char.contains(&'"') {
            self.advance_one();

            self.advance_while(|iter| {
                if let Some(at_char) = iter.at_char {
                    at_char != '"' && !iter.last_char.contains(&'\\')
                } else {
                    false
                }
            });
        }

        self.advance_while_is_whitespace_is(false);

        if start_at == self.at_pos {
            return None;
        }
        Some(&self.line[start_at..self.at_pos])
    }
}
