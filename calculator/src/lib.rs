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

impl<T> ParseTree<T> {
    fn as_internal(&self) -> Option<(&parser::Nonterminal, &Vec<ParseTree<T>>)> {
        if let ParseTree::Internal(nterm, children) = self {
            return Some((&nterm, &children));
        }
        None
    }

    fn as_leaf(&self) -> Option<&parser::Token<T>> {
        if let ParseTree::Leaf(term) = self {
            return Some(term);
        }
        None
    }
}

fn evaluate_from_string(expr: &str) -> Option<f64> {
    let mut lexer = Lexer::new(expr);
    let tokens = lexer.get_tokens().unwrap();
    let tables = parser::get_parse_tables();
    let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
    evaluate_from_tree(&tree)
}

fn evaluate_from_tree(tree: &ParseTree<TokenAttribute>) -> Option<f64> {
    let (_, children) = tree.as_internal()?;
    let res1 = evaluate_from_tree_t(children.get(0)?)?;
    let res2 = evaluate_from_tree_et(children.get(1)?)?;
    Some(res1 + res2)
}

fn evaluate_from_tree_t(tree: &ParseTree<TokenAttribute>) -> Option<f64> {
    let (_, children) = tree.as_internal()?;
    let res1 = evaluate_from_tree_f(children.get(0)?)?;
    let res2 = evaluate_from_tree_tt(children.get(1)?)?;
    Some(res1 * res2)
}

fn evaluate_from_tree_f(tree: &ParseTree<TokenAttribute>) -> Option<f64> {
    let (_, children) = tree.as_internal()?;
    if children.len() == 1 {
        let c = children[0].as_leaf()?;
        let n = c.attribute.domain_attribute.as_number()?;
        Some(f64::from(n))
    } else {
        let c = children.get(1)?;
        evaluate_from_tree(c)
    }
}

fn evaluate_from_tree_tt(tree: &ParseTree<TokenAttribute>) -> Option<f64> {
    let (_, children) = tree.as_internal()?;
    if children.len() == 0 {
        Some(1.0)
    } else {
        let sign = children.get(0)?.as_leaf()?.tag.as_terminal()?.0.clone();
        let res1 = evaluate_from_tree_f(children.get(1)?)?;
        let res2 = evaluate_from_tree_tt(children.get(2)?)?;
        let res = res1 * res2;
        if sign == "/" {
            if res == 0.0 {
                return None;
            }
            return Some(1.0/res);
        }
        Some(res)
    }
}

fn evaluate_from_tree_et(tree: &ParseTree<TokenAttribute>) -> Option<f64> {
    let (_, children) = tree.as_internal()?;
    if children.len() == 0 {
        Some(0.0)
    } else {
        let sign = children.get(0)?.as_leaf()?.tag.as_terminal()?.0.clone();
        let res1 = evaluate_from_tree_t(children.get(1)?)?;
        let res2 = evaluate_from_tree_et(children.get(2)?)?;
        let res = res1 + res2;
        if sign == "-" {
            return Some(-res);
        }
        Some(res)
    }
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

impl DomainAttribute {
    fn as_number(&self) -> Option<i32> {
        if let DomainAttribute::Number(n) = self {
            return Some(n.clone());
        }
        None
    }
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

impl TerminalOrFinish {
    fn as_terminal(&self) -> Option<&parser::Terminal> {
        if let TerminalOrFinish::Terminal(t) = self {
            return Some(t);
        }
        None
    }
}