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

#[derive(Debug)]
pub struct ParseError<'a, T> {
    pub token: &'a Token<T>,
}

fn err_on_none<T, P>(res: Option<T>, token: &Token<P>) -> Result<T, ParseError<P>> {
    match res {
        Some(v) => Ok(v),
        None => Err(ParseError { token }),
    }
}

impl<T: Clone> ParseTree<T> {
    pub fn from_tables_and_tokens<'a, 'b>(
        tables: &'a ParseTables,
        tokens: &'b [Token<T>],
    ) -> Result<ParseTree<T>, ParseError<'b, T>> {
        let mut states = vec![tables.start];
        let mut trees: Vec<ParseTree<T>> = Vec::new();
        let mut token_index = 0;
        loop {
            let token = &tokens[token_index];
            let cur_state = err_on_none(states.last(), token)?;
            let action = err_on_none(
                tables.action.get(&(cur_state.clone(), token.tag.clone())),
                token,
            )?;
            match action {
                LR1Action::Shift(state) => {
                    states.push(state.clone());
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
                    let next =
                        err_on_none(tables.goto.get(&(cur.clone(), rule.left.clone())), token)?;
                    states.push(next.clone());
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
        ((8, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (
                5,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (
                20,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(14),
        ),
        (
            (
                9,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(19),
        ),
        (
            (
                14,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(18),
        ),
        (
            (
                10,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(9),
        ),
        (
            (
                18,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(0),
        ),
        (
            (
                5,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(6),
        ),
        (
            (
                4,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("P")),
                right: vec![],
            }),
        ),
        (
            (17, TerminalOrFinish::Finish),
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
                19,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(5),
        ),
        (
            (
                7,
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
                12,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(9),
        ),
        (
            (
                4,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(5),
        ),
        (
            (
                0,
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
            (16, TerminalOrFinish::Finish),
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
                6,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(6),
        ),
        (
            (
                22,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(7),
        ),
        (
            (
                21,
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
            (7, TerminalOrFinish::Finish),
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
                5,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (
                1,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(23),
        ),
        (
            (0, TerminalOrFinish::Finish),
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
                15,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(4),
        ),
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(6),
        ),
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
            (10, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                11,
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
                13,
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
                6,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (12, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                6,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("ax")))),
            LR1Action::Shift(2),
        ),
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (
                2,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(20),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((3, Nonterminal(String::from("I"))), 13),
        ((1, Nonterminal(String::from("A"))), 10),
        ((10, Nonterminal(String::from("T"))), 12),
        ((19, Nonterminal(String::from("P"))), 22),
        ((10, Nonterminal(String::from("R"))), 17),
        ((4, Nonterminal(String::from("P"))), 21),
        ((12, Nonterminal(String::from("R"))), 16),
        ((1, Nonterminal(String::from("S"))), 8),
        ((12, Nonterminal(String::from("T"))), 12),
        ((5, Nonterminal(String::from("I"))), 15),
        ((6, Nonterminal(String::from("I"))), 11),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 1,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
