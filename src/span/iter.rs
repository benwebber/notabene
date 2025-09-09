use super::Span;

pub struct SpanIterator<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> SpanIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, cursor: 0 }
    }
}

impl<'a> Iterator for SpanIterator<'a> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.text[self.cursor..]
            .char_indices()
            .find(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| self.cursor + i);
        let Some(start) = start else {
            // End of string.
            self.cursor = self.text.len();
            return None;
        };
        let end = self.text[start..]
            .char_indices()
            .find(|(_, c)| c.is_whitespace())
            .map(|(i, _)| start + i)
            .unwrap_or(self.text.len());
        self.cursor = end;
        Some(Span::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_iterator() {
        let s = "  foo  foobar  quux  ";
        let spans: Vec<Span> = SpanIterator::new(s).collect();
        assert_eq!(
            spans,
            vec![Span::new(2, 5), Span::new(7, 13), Span::new(15, 19)]
        );
    }
}
