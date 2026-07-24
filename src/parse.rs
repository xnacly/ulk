use std::fmt::Display;

use crate::lex;

pub struct Parser<'parse> {
    lex: lex::Lex<'parse>,
    cur: lex::Token<'parse>,
}

#[derive(Debug, PartialEq)]
pub enum Node<'node> {
    Binding(&'node str),
    Lambda((&'node str, Box<Node<'node>>)),
}

impl<'parse> Parser<'parse> {
    pub fn new(src: &'parse [u8]) -> Parser<'parse> {
        let mut lex = lex::Lex::new(src);
        let cur = lex.next().unwrap_or_default();
        Self { lex, cur }
    }

    fn advance(&mut self) {
        self.cur = self.lex.next().unwrap_or_default();
    }

    pub fn parse_one(&mut self) -> Option<Node<'parse>> {
        match self.cur {
            lex::Token::Lambda => {
                self.advance();
                let lex::Token::Binding(txt) = self.cur else {
                    panic!("Lambda calc mandates lx.x, missing function argument");
                };
                let argument = unsafe { str::from_utf8_unchecked(&txt) };

                self.advance();
                let lex::Token::Dot = self.cur else {
                    panic!("Lambda calc mandates lx.x, missing dot");
                };

                self.advance();
                let body = Box::new(self.parse_one()?);
                Some(Node::Lambda((argument, body)))
            }
            lex::Token::Binding(txt) => {
                self.advance();
                Some(Node::Binding(unsafe { str::from_utf8_unchecked(&txt) }))
            }
            lex::Token::Eof => None,
            _ => unreachable!(),
        }
    }
}

impl<'parse> Iterator for Parser<'parse> {
    type Item = Node<'parse>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_one()
    }
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl Node<'_> {
    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        write!(f, "{}", " ".repeat(indent))?;

        match self {
            Node::Binding(binding) => write!(f, "{binding}"),
            Node::Lambda((arg, body)) => {
                writeln!(f, "l:")?;
                write!(f, "{}{}", " ".repeat(indent + 1), arg)?;
                writeln!(f)?;
                body.fmt_with_indent(f, indent + 1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Node, Parser};

    fn parse(src: &[u8]) -> Vec<Node<'_>> {
        Parser::new(src).collect()
    }

    #[test]
    fn parses_identity_lambda() {
        assert_eq!(
            parse(b"lx.x"),
            vec![Node::Lambda((
                "x",
                Box::new(Node::Binding("x"))
            ))]
        );
    }

    #[test]
    fn parses_nested_lambda_body() {
        assert_eq!(
            parse(b"lx.ly.y"),
            vec![Node::Lambda((
                "x",
                Box::new(Node::Lambda((
                    "y",
                    Box::new(Node::Binding("y")),
                ))),
            ))]
        );
    }

    #[test]
    fn parses_top_level_bindings() {
        assert_eq!(
            parse(b"foo bar"),
            vec![Node::Binding("foo"), Node::Binding("bar")]
        );
    }

    #[test]
    fn parses_empty_input() {
        assert_eq!(parse(b""), vec![]);
    }

    #[test]
    fn skips_comments_and_whitespace() {
        assert_eq!(
            parse(b"# identity\nlx.x\n# done"),
            vec![Node::Lambda((
                "x",
                Box::new(Node::Binding("x"))
            ))]
        );
    }

    #[test]
    #[should_panic(expected = "missing function argument")]
    fn rejects_lambda_without_argument() {
        parse(b"l.x");
    }

    #[test]
    #[should_panic(expected = "missing dot")]
    fn rejects_lambda_without_dot() {
        parse(b"lx x");
    }
}
