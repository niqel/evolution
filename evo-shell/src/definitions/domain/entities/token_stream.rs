#[derive(Debug)]
pub struct TokenStream<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub(crate) fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }

    pub(crate) fn advance_to(&mut self, position: usize) {
        debug_assert!(self.input.is_char_boundary(position));
        self.position = position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_stream_borrows_input_and_starts_at_initial_position() {
        let input = "scope-fs \"/tmp\"";
        let stream = TokenStream::new(input);

        assert_eq!(stream.input(), input);
        assert_eq!(stream.position(), 0);
    }
}
