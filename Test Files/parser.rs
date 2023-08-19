use std::process::{EXIT_FAILURE, self};


use crate::tokenization::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

#[derive()]
pub(crate) struct NodeExit {
    expr: NodeIntLit,
}

impl NodeExit {
    pub(crate) fn new(expr: NodeIntLit) -> Self { Self { expr } }
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node = None;
        while let Some(token) = self.peek(0) {
            if token.token_type == TokenType::Exit {
                self.consume();
                if let Some(node_expr) = self.parse_expr() {
                    exit_node = Some(NodeExit { expr: node_expr });
                } else {
                    eprintln!("Invalid expression");
                    std::process::exit(EXIT_FAILURE);
                }
                if let Some(token) = self.peek(0) {
                    if token.token_type == TokenType::Semi {
                        self.consume();
                    } else {
                        eprintln!("Invalid expression");
                        std::process;
                        pub const EXIT_FAILURE: i32 = process;
                        std::process::exit(EXIT_FAILURE);
                    }
                }
            } else {
                self.consume();
            }
        }
        self.index = 0;
        exit_node
    }

    fn peek(&self, ahead: usize) -> Option<&Token> {
        self.tokens.get(self.index + ahead)
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.index];
        self.index += 1;
        token
    }
}
