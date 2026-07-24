use std::collections::HashMap;

use crate::{Options, parse};

#[derive(Default)]
pub struct Env<'env> {
    bindings: HashMap<&'env str, ()>,
}

pub struct Ulk<'ulk> {
    opts: &'ulk Options,
    parser: parse::Parser<'ulk>,
    envs: Vec<Env<'ulk>>,
}

impl<'ulk> Ulk<'ulk> {
    pub fn new(opts: &'ulk Options, src: &'ulk [u8]) -> Ulk<'ulk> {
        Self {
            parser: parse::Parser::new(src),
            envs: {
                let mut envs = Vec::with_capacity(64);
                envs.push(Env::default());
                envs
            },
            opts,
        }
    }

    fn next_node(&mut self) -> Option<parse::Node<'ulk>> {
        self.parser.parse_one()
    }

    fn find_bindings(&self, name: &'ulk str) -> Option<&Env<'ulk>> {
        self.envs
            .iter()
            .rev()
            .find(|x| x.bindings.contains_key(name))
    }

    fn eval_node(&mut self, node: &parse::Node<'ulk>) -> Result<(), ()> {
        match node {
            parse::Node::Binding(name) => {
                let Some(_bound) = self.find_bindings(name) else {
                    panic!("no binding called {name} found in the env stack");
                };
            }
            parse::Node::Lambda((arg, body)) => {
                if let Some(curenv) = self.envs.last_mut() {
                    curenv.bindings.insert(arg, ());
                }
                self.eval_node(body)?;
            }
        }
        Ok(())
    }

    pub fn eval(&mut self) -> Result<(), ()> {
        while let Some(node) = self.next_node() {
            if self.opts.ast {
                println!("{}", node)
            }

            self.eval_node(&node)?;
        }
        Ok(())
    }
}
