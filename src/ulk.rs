use crate::{Options, parse};

pub struct Ulk<'ulk> {
    opts: &'ulk Options,
    parser: parse::Parser<'ulk>,
}

impl<'ulk> Ulk<'ulk> {
    pub fn new(opts: &'ulk Options, src: &'ulk [u8]) -> Ulk<'ulk> {
        Self {
            parser: parse::Parser::new(src),
            opts,
        }
    }

    fn next_node(&mut self) -> Option<parse::Node<'ulk>> {
        self.parser.parse_one()
    }

    pub fn eval(&mut self) -> Result<(), ()> {
        while let Some(node) = self.next_node() {
            if self.opts.ast {
                println!("{}", node)
            }
        }
        todo!("skibidi");
        Ok(())
    }
}
