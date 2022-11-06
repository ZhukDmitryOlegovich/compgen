use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Term {
    Nonterminal(Nonterminal),
    Terminal(Terminal),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Rule {
    pub left: Nonterminal,
    pub right: Vec<Term>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LR1Action {
    Reduce(Rule),
    Shift(i32),
    Accept,
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseTables {
    pub start: i32,
    pub action: HashMap<(i32, TerminalOrFinish), LR1Action>,
    pub goto: HashMap<(i32, Nonterminal), i32>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

impl ToString for TerminalOrFinish {
    fn to_string(&self) -> String {
        match self {
            TerminalOrFinish::Terminal(t) => t.0.clone(),
            TerminalOrFinish::Finish => String::from("$"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct Nonterminal(pub String);
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]

pub struct Terminal(pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token<T> {
    pub tag: TerminalOrFinish,
    pub attribute: T,
}

#[derive(Debug)]
pub enum ParseTree<T> {
    Internal(Nonterminal, Vec<ParseTree<T>>),
    Leaf(Token<T>),
}

impl<T> ParseTree<T> {
    pub fn as_internal(&self) -> Option<(&Nonterminal, &Vec<ParseTree<T>>)> {
        if let ParseTree::Internal(nterm, children) = self {
            return Some((nterm, children));
        }
        None
    }

    pub fn as_leaf(&self) -> Option<&Token<T>> {
        if let ParseTree::Leaf(term) = self {
            return Some(term);
        }
        None
    }
}

#[derive(Debug)]
pub struct ParseError<T> {
    pub token: Token<T>,
}

fn err_on_none<T, P: Clone>(res: Option<T>, token: &Token<P>) -> Result<T, ParseError<P>> {
    match res {
        Some(v) => Ok(v),
        None => Err(ParseError {
            token: token.clone(),
        }),
    }
}

impl<T: Clone> ParseTree<T> {
    pub fn from_tables_and_tokens(
        tables: &ParseTables,
        tokens: &[Token<T>],
    ) -> Result<ParseTree<T>, ParseError<T>> {
        let mut states = vec![tables.start];
        let mut trees: Vec<ParseTree<T>> = Vec::new();
        let mut token_index = 0;
        loop {
            let token = &tokens[token_index];
            let cur_state = err_on_none(states.last(), token)?;
            let action = err_on_none(tables.action.get(&(*cur_state, token.tag.clone())), token)?;
            match action {
                LR1Action::Shift(state) => {
                    states.push(*state);
                    trees.push(Self::Leaf(token.clone()));
                    token_index += 1;
                }
                LR1Action::Reduce(rule) => {
                    let mut children: Vec<ParseTree<T>> = Vec::new();
                    for _ in 0..rule.right.len() {
                        states.pop();
                        children.push(err_on_none(trees.pop(), token)?);
                    }
                    children.reverse();
                    trees.push(ParseTree::Internal(rule.left.clone(), children));
                    let cur = err_on_none(states.last(), token)?;
                    let next = err_on_none(tables.goto.get(&(*cur, rule.left.clone())), token)?;
                    states.push(*next);
                }
                LR1Action::Accept => {
                    return err_on_none(trees.pop(), token);
                }
            }
        }
    }

    pub fn to_graphviz(&self) -> String {
        let mut counter = 0;
        let inner = self.to_graphviz_rec(&mut counter);
        let mut res = String::new();
        res += "digraph G {\n";
        res += inner.as_ref();
        res += "}\n";
        res
    }

    pub fn to_graphviz_rec(&self, counter: &mut i32) -> String {
        *counter += 1;
        let id = *counter;
        let mut result = String::new();
        match self {
            ParseTree::Internal(nterm, children) => {
                result += format!("{} [label=\"{}\"]\n", id, nterm.0).as_ref();
                for child in children {
                    let child_id = *counter + 1;
                    result += format!("{id} -> {child_id}\n").as_ref();
                    result += child.to_graphviz_rec(counter).as_ref();
                }
            }
            ParseTree::Leaf(token) => {
                result += format!("{} [label=\"{}\"]\n", id, token.tag.to_string()).as_ref();
            }
        }
        result
    }
}

//@START_PARSE_TABLES@

pub fn get_parse_tables() -> ParseTables {
    let action = [
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (
                4,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(5),
        ),
        (
            (
                7,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(17),
        ),
        (
            (11, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                17,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (
                19,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("P")),
                right: vec![],
            }),
        ),
        (
            (
                19,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(17),
        ),
        (
            (
                14,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(4),
        ),
        (
            (
                8,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                ],
            }),
        ),
        (
            (
                20,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (22, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("S")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("A"))),
                    Term::Nonterminal(Nonterminal(String::from("R"))),
                ],
            }),
        ),
        (
            (
                21,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(6),
        ),
        (
            (
                6,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(7),
        ),
        (
            (
                7,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("P")),
                right: vec![],
            }),
        ),
        (
            (
                18,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(10),
        ),
        (
            (1, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Nonterminal(Nonterminal(String::from("P"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            }),
        ),
        (
            (
                9,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(19),
        ),
        (
            (5, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("A")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("ax"))),
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Terminal(Terminal(String::from("close"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            }),
        ),
        (
            (
                17,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(20),
        ),
        (
            (
                0,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(1),
        ),
        ((2, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (
                15,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(16),
        ),
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(20),
        ),
        (
            (
                20,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(20),
        ),
        (
            (
                16,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(14),
        ),
        (
            (13, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("R"))),
                ],
            }),
        ),
        (
            (
                1,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Nonterminal(Nonterminal(String::from("P"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            }),
        ),
        (
            (
                23,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("P")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                    Term::Terminal(Terminal(String::from("close"))),
                    Term::Nonterminal(Nonterminal(String::from("P"))),
                ],
            }),
        ),
        (
            (
                17,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (21, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                20,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from("ax")))),
            LR1Action::Shift(15),
        ),
        (
            (
                5,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("A")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("ax"))),
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Terminal(Terminal(String::from("close"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            }),
        ),
        (
            (
                12,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![
                    Term::Terminal(Terminal(String::from("term"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                ],
            }),
        ),
        (
            (
                11,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(6),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((18, Nonterminal(String::from("S"))), 2),
        ((18, Nonterminal(String::from("A"))), 21),
        ((21, Nonterminal(String::from("T"))), 11),
        ((19, Nonterminal(String::from("P"))), 23),
        ((17, Nonterminal(String::from("I"))), 9),
        ((20, Nonterminal(String::from("I"))), 12),
        ((11, Nonterminal(String::from("T"))), 11),
        ((3, Nonterminal(String::from("I"))), 8),
        ((11, Nonterminal(String::from("R"))), 13),
        ((21, Nonterminal(String::from("R"))), 22),
        ((7, Nonterminal(String::from("P"))), 0),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 18,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
