pub mod parser;
mod tests;

use parser::ParseError;

use crate::parser::{ParseTree, Terminal, TerminalOrFinish, Token};

pub struct Lexer {
    cur: Coordinate,
    input: String,
}

#[derive(Debug)]
pub enum CalculatorError {
    LexerError(Coordinate),
    ParserError(ParseError<TokenAttribute>),
    ZeroDivisionError,
}

impl<'a> From<ParseError<TokenAttribute>> for CalculatorError {
    fn from(err: ParseError<TokenAttribute>) -> Self {
        CalculatorError::ParserError(err)
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            cur: Coordinate {
                line: 1,
                column: 1,
                index: 0,
            },
            input: String::from(input),
        }
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token<TokenAttribute>>, CalculatorError> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token()?;
            res.push(token.clone());
            if let TerminalOrFinish::Finish = token.tag {
                break;
            }
        }
        Ok(res)
    }

    fn get_next_token(&mut self) -> Result<Token<TokenAttribute>, CalculatorError> {
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
                    return Ok(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("n"))),
                        attribute: TokenAttribute {
                            fragment: Fragment { begin, end },
                            domain_attribute: DomainAttribute::Number(n),
                        },
                    });
                } else if ['+', '-', '*', '/', '(', ')'].contains(&cur) {
                    self.next();
                    let end = self.cur.clone();
                    return Ok(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from(String::from(cur)))),
                        attribute: TokenAttribute {
                            fragment: Fragment { begin, end },
                            domain_attribute: DomainAttribute::None,
                        },
                    });
                } else {
                    return Err(CalculatorError::LexerError(self.cur.clone()));
                }
            }
            None => Ok(Token {
                tag: TerminalOrFinish::Finish,
                attribute: TokenAttribute {
                    fragment: Fragment {
                        begin: begin.clone(),
                        end: begin.clone(),
                    },
                    domain_attribute: DomainAttribute::None,
                },
            }),
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

pub fn evaluate_from_string(expr: &str) -> Result<f64, CalculatorError> {
    let mut lexer = Lexer::new(expr);
    let tokens = lexer.get_tokens()?;
    let tables = parser::get_parse_tables();
    let tree = ParseTree::from_tables_and_tokens(&tables, &tokens)?;
    evaluate_from_tree(&tree)
}

// <E <T E'>>
fn evaluate_from_tree(tree: &ParseTree<TokenAttribute>) -> Result<f64, CalculatorError> {
    let (_, children) = tree.as_internal().expect("must be internal node");
    let res1 = evaluate_from_tree_t(&children[0])?;
    let res2 = evaluate_from_tree_et(&children[1])?;
    Ok(res1 + res2)
}

//<E' <+ T E'>
//    <- T E'>
//    <>>
fn evaluate_from_tree_et(tree: &ParseTree<TokenAttribute>) -> Result<f64, CalculatorError> {
    let (_, children) = tree.as_internal().expect("must be internal node");
    if children.len() == 0 {
        Ok(0.0)
    } else {
        let sign = children[0]
            .as_leaf()
            .expect("must be leaf node")
            .tag
            .as_terminal()
            .expect("must be a terminal")
            .0
            .clone();
        let res1 = evaluate_from_tree_t(&children[1])?;
        let res2 = evaluate_from_tree_et(&children[2])?;
        let res = res1 + res2;
        if sign == "-" {
            return Ok(-res);
        }
        Ok(res)
    }
}

// <T <F T'>>
fn evaluate_from_tree_t(tree: &ParseTree<TokenAttribute>) -> Result<f64, CalculatorError> {
    let (_, children) = tree.as_internal().expect("must be internal node");
    let res1 = evaluate_from_tree_f(&children[0])?;
    let res2 = evaluate_from_tree_tt(&children[1])?;
    Ok(res1 * res2)
}

// <T' <* F T'>
//     </ F T'>
//     <>>
fn evaluate_from_tree_tt(tree: &ParseTree<TokenAttribute>) -> Result<f64, CalculatorError> {
    let (_, children) = tree.as_internal().expect("must be internal node");
    if children.len() == 0 {
        Ok(1.0)
    } else {
        let sign = children[0]
            .as_leaf()
            .expect("must be leaf node")
            .tag
            .as_terminal()
            .expect("must be terminal")
            .0
            .clone();
        let res1 = evaluate_from_tree_f(&children[1])?;
        let res2 = evaluate_from_tree_tt(&children[2])?;
        let res = res1 * res2;
        if sign == "/" {
            if res == 0.0 {
                return Err(CalculatorError::ZeroDivisionError);
            }
            return Ok(1.0 / res);
        }
        Ok(res)
    }
}

// <F <n>
//    <( E )>>
fn evaluate_from_tree_f(tree: &ParseTree<TokenAttribute>) -> Result<f64, CalculatorError> {
    let (_, children) = tree.as_internal().expect("must be internal node");
    if children.len() == 1 {
        let c = children[0].as_leaf().expect("must be leaf node");
        let n = c
            .attribute
            .domain_attribute
            .as_number()
            .expect("must be number");
        Ok(f64::from(n))
    } else {
        let c = &children[1];
        evaluate_from_tree(c)
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
