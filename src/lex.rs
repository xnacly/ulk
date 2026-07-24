pub(crate) struct Lex<'l> {
    src: &'l [u8],
    pos: usize,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) enum Token<'l> {
    Binding(&'l [u8]),
    Lambda,
    Dot,
    #[default]
    Eof,
}

impl<'l> Lex<'l> {
    pub(crate) fn new(src: &'l [u8]) -> Lex<'l> {
        Lex { src, pos: 0 }
    }

    fn cur(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let cur = self.cur()?;
        self.pos += 1;
        Some(cur)
    }

    fn take_while(&mut self, f: impl Fn(u8) -> bool) -> &'l [u8] {
        let start = self.pos;

        while let Some(cur) = self.cur() {
            if !f(cur) {
                break;
            }
            self.pos += 1;
        }

        &self.src[start..self.pos]
    }

    fn skip_ignored(&mut self) {
        while let Some(cur) = self.cur() {
            match cur {
                b'#' => {
                    self.take_while(|cur| cur != b'\n');
                }
                cur if cur.is_ascii_whitespace() => {
                    self.bump();
                }
                _ => break,
            }
        }
    }
}

impl<'l> Iterator for Lex<'l> {
    type Item = Token<'l>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_ignored();

        match self.cur()? {
            b'l' => {
                self.bump();
                Some(Token::Lambda)
            }
            b'.' => {
                self.bump();
                Some(Token::Dot)
            }
            b'a'..=b'z' | b'A'..=b'Z' => Some(Token::Binding(
                self.take_while(|cur| cur.is_ascii_alphabetic()),
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lex, Token};

    fn assert_lexes(src: &str, tokens: Vec<Token<'_>>) {
        assert_eq!(Lex::new(src.as_bytes()).collect::<Vec<_>>(), tokens);
    }

    #[test]
    fn tokenizes_lambda_expression() {
        assert_lexes(
            "lx.x",
            vec![
                Token::Lambda,
                Token::Binding(b"x"),
                Token::Dot,
                Token::Binding(b"x"),
            ],
        );
    }

    #[test]
    fn tokenizes_bindings() {
        assert_lexes(
            "foo Bar z Z",
            vec![
                Token::Binding(b"foo"),
                Token::Binding(b"Bar"),
                Token::Binding(b"z"),
                Token::Binding(b"Z"),
            ],
        );
    }

    #[test]
    fn ignores_whitespace_and_comments() {
        assert_lexes(
            " \n\t# comment\r\nfoo # another comment\n\n# trailing comment",
            vec![Token::Binding(b"foo")],
        );
    }

    #[test]
    fn handles_empty_input() {
        assert_lexes("", vec![]);
    }

    #[test]
    fn stops_on_unknown_characters() {
        assert_lexes("foo@bar", vec![Token::Binding(b"foo")]);
    }

    #[test]
    fn rejects_lone_minus() {
        assert_lexes("-", vec![]);
    }
}
