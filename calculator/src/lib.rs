pub mod parser;
mod tests;

use crate::parser::{Terminal, TerminalOrFinish, Token, ParseTree};

pub struct Lexer {
    cur: Coordinate,
    input: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            cur: Coordinate { line: 1, column: 1, index: 0 },
            input: String::from(input),
        }
    }

    pub fn get_tokens(&mut self) -> Option<Vec<Token<TokenAttribute>>> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token()?;
            res.push(token.clone());
            if let TerminalOrFinish::Finish = token.tag {
                break;
            }
        }
        Some(res)
    }

    fn get_next_token(&mut self) -> Option<Token<TokenAttribute>> {
        self.skip_spaces();
        let begin = self.cur.clone();
        match self.peek() {
            Some(cur) => {
                if cur.is_ascii_digit() {
                    let n = self
                        .read_while(|x| x.is_ascii_digit())
                        .parse::<i32>()
                        .unwrap();
                    let end = self.cur.clone();
                    return Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("n"))),
                        attribute: TokenAttribute {
                            fragment: Fragment { begin, end },
                            domain_attribute: DomainAttribute::Number(n),
                        },
                    });
                } else if ['+', '-', '*', '/', '(', ')'].contains(&cur) {
                    self.next();
                    let end = self.cur.clone();
                    return Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from(String::from(cur)))),
                        attribute: TokenAttribute {
                            fragment: Fragment { begin, end },
                            domain_attribute: DomainAttribute::None,
                        },
                    });
                } else {
                    None
                }
            }
            None => Some(Token {
                tag: TerminalOrFinish::Finish,
                attribute: TokenAttribute { fragment: Fragment {
                    begin: begin.clone(),
                    end: begin.clone(),
                }, domain_attribute: DomainAttribute::None, }
            })
        }
    }

    fn skip_spaces(&mut self) {
        self.read_while(|x| x.is_whitespace());
    }

    fn read_while<F>(&mut self, p: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        loop {
            match self.peek() {
                Some(c) if p(c) => {
                    res.push(c);
                    self.next();
                }
                _ => break,
            }
        }
        res
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cur.index as usize)
    }

    fn next(&mut self) {
        if let Some(c) = self.peek() {
            self.cur.column += 1;
            if c == '\n' {
                self.cur.line += 1;
                self.cur.column = 1;
            }
            self.cur.index += 1;
        }
    }
}

fn evaluate_from_tree(tree: ParseTree<TokenAttribute>) -> f64 {
    0.0
}

#[derive(Clone, Debug)]
pub struct TokenAttribute {
    fragment: Fragment,
    domain_attribute: DomainAttribute,
}

#[derive(Clone, Debug)]
pub enum DomainAttribute {
    Number(i32),
    None,
}

#[derive(Clone, Debug)]
pub struct Fragment {
    begin: Coordinate,
    end: Coordinate,
}

#[derive(Clone, Debug)]
pub struct Coordinate {
    line: i32,
    column: i32,
    index: i32,
}
