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
            (3, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(8),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (23, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (28, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (31, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(5),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(24),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(34),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(34),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (11, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (6, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
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
            (36, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(34),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (28, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(27),
        ),
        ((35, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (8, TerminalOrFinish::Finish),
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
            (14, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(21),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (21, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(27),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (29, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(30),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (28, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(31),
        ),
        (
            (40, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (15, TerminalOrFinish::Finish),
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
            (34, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (39, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(24),
        ),
        (
            (22, TerminalOrFinish::Finish),
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
            (4, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(17),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(21),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (30, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (17, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(27),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (30, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (27, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (25, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (13, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (41, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(30),
        ),
        (
            (16, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(14),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (36, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(31),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(30),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (13, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(5),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (12, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(11),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(27),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (40, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(5),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(17),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (14, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (28, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (20, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(24),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(31),
        ),
        (
            (28, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(17),
        ),
        (
            (38, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (9, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(4),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (0, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (40, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(21),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (31, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(27),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((40, Nonterminal(String::from("E'"))), 25),
        ((24, Nonterminal(String::from("F"))), 7),
        ((36, Nonterminal(String::from("E"))), 3),
        ((32, Nonterminal(String::from("T'"))), 18),
        ((17, Nonterminal(String::from("F"))), 28),
        ((23, Nonterminal(String::from("T'"))), 2),
        ((30, Nonterminal(String::from("T"))), 29),
        ((4, Nonterminal(String::from("F"))), 7),
        ((20, Nonterminal(String::from("T'"))), 1),
        ((37, Nonterminal(String::from("F"))), 20),
        ((0, Nonterminal(String::from("T"))), 40),
        ((31, Nonterminal(String::from("F"))), 23),
        ((29, Nonterminal(String::from("E'"))), 38),
        ((21, Nonterminal(String::from("T"))), 41),
        ((30, Nonterminal(String::from("F"))), 7),
        ((41, Nonterminal(String::from("E'"))), 15),
        ((28, Nonterminal(String::from("T'"))), 19),
        ((9, Nonterminal(String::from("T'"))), 16),
        ((4, Nonterminal(String::from("E"))), 12),
        ((21, Nonterminal(String::from("F"))), 9),
        ((36, Nonterminal(String::from("T"))), 33),
        ((7, Nonterminal(String::from("T'"))), 39),
        ((6, Nonterminal(String::from("E'"))), 26),
        ((5, Nonterminal(String::from("F"))), 9),
        ((0, Nonterminal(String::from("F"))), 9),
        ((33, Nonterminal(String::from("E'"))), 10),
        ((0, Nonterminal(String::from("E"))), 35),
        ((24, Nonterminal(String::from("T"))), 6),
        ((34, Nonterminal(String::from("F"))), 32),
        ((5, Nonterminal(String::from("T"))), 13),
        ((13, Nonterminal(String::from("E'"))), 22),
        ((36, Nonterminal(String::from("F"))), 7),
        ((4, Nonterminal(String::from("T"))), 33),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 0,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
