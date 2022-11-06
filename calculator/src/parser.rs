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
            (20, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(6),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (20, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(15),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(6),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("*"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(0),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(19),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("+"))),
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (4, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("+"))),
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (2, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("*"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("-"))),
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (17, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(6),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("*"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("*"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (5, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (7, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(13),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(13),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(15),
        ),
        ((11, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(3),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (18, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(1),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (9, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (10, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("-"))),
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(0),
        ),
        (
            (3, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(15),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (12, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (19, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![
                    Term::Terminal(Terminal(String::from("("))),
                    Term::Nonterminal(Nonterminal(String::from("E"))),
                    Term::Terminal(Terminal(String::from(")"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(13),
        ),
        (
            (14, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(0),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((6, Nonterminal(String::from("F"))), 16),
        ((21, Nonterminal(String::from("E"))), 11),
        ((1, Nonterminal(String::from("F"))), 20),
        ((0, Nonterminal(String::from("F"))), 14),
        ((21, Nonterminal(String::from("F"))), 20),
        ((13, Nonterminal(String::from("F"))), 20),
        ((15, Nonterminal(String::from("T"))), 7),
        ((7, Nonterminal(String::from("E'"))), 4),
        ((14, Nonterminal(String::from("T'"))), 12),
        ((16, Nonterminal(String::from("T'"))), 2),
        ((20, Nonterminal(String::from("T'"))), 9),
        ((13, Nonterminal(String::from("T"))), 18),
        ((5, Nonterminal(String::from("E'"))), 17),
        ((15, Nonterminal(String::from("F"))), 20),
        ((1, Nonterminal(String::from("T"))), 5),
        ((21, Nonterminal(String::from("T"))), 5),
        ((18, Nonterminal(String::from("E'"))), 10),
        ((1, Nonterminal(String::from("E"))), 8),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 21,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
