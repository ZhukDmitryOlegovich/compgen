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
        (
            (28, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (12, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(8),
        ),
        (
            (38, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(14),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(8),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(22),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (39, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(8),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (7, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            LR1Action::Shift(4),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(38),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (29, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (0, TerminalOrFinish::Finish),
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
            (3, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(9),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (9, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(22),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(37),
        ),
        (
            (32, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (26, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(9),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(13),
        ),
        ((1, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(9),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (21, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(14),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(22),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (12, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (7, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(14),
        ),
        (
            (22, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(15),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(38),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(13),
        ),
        (
            (35, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(10),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (19, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(37),
        ),
        (
            (21, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (18, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(37),
        ),
        (
            (38, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (33, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (15, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(37),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (5, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (20, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (28, TerminalOrFinish::Finish),
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
            (30, TerminalOrFinish::Finish),
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
            (10, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(18),
        ),
        (
            (3, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            LR1Action::Shift(15),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (34, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (14, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(37),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(22),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(22),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (33, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(15),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(13),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(6),
        ),
        (
            (28, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (3, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(10),
        ),
        (
            (40, TerminalOrFinish::Finish),
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
            (31, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(5),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(36),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            LR1Action::Shift(6),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (7, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(38),
        ),
        (
            (5, TerminalOrFinish::Finish),
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
            (27, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (2, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(10),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(6),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((9, Nonterminal(String::from("T"))), 2),
        ((26, Nonterminal(String::from("E'"))), 17),
        ((10, Nonterminal(String::from("T"))), 26),
        ((21, Nonterminal(String::from("T'"))), 3),
        ((37, Nonterminal(String::from("T"))), 16),
        ((27, Nonterminal(String::from("T'"))), 28),
        ((39, Nonterminal(String::from("T'"))), 24),
        ((2, Nonterminal(String::from("E'"))), 20),
        ((36, Nonterminal(String::from("T"))), 16),
        ((37, Nonterminal(String::from("F"))), 7),
        ((6, Nonterminal(String::from("F"))), 21),
        ((9, Nonterminal(String::from("F"))), 7),
        ((37, Nonterminal(String::from("E"))), 31),
        ((12, Nonterminal(String::from("T'"))), 34),
        ((36, Nonterminal(String::from("E"))), 11),
        ((16, Nonterminal(String::from("E'"))), 25),
        ((15, Nonterminal(String::from("T"))), 29),
        ((33, Nonterminal(String::from("E'"))), 30),
        ((32, Nonterminal(String::from("E'"))), 35),
        ((29, Nonterminal(String::from("E'"))), 40),
        ((6, Nonterminal(String::from("T"))), 33),
        ((8, Nonterminal(String::from("F"))), 19),
        ((23, Nonterminal(String::from("T"))), 32),
        ((36, Nonterminal(String::from("F"))), 7),
        ((14, Nonterminal(String::from("F"))), 27),
        ((10, Nonterminal(String::from("F"))), 7),
        ((19, Nonterminal(String::from("T'"))), 0),
        ((13, Nonterminal(String::from("F"))), 12),
        ((23, Nonterminal(String::from("F"))), 21),
        ((7, Nonterminal(String::from("T'"))), 41),
        ((23, Nonterminal(String::from("E"))), 1),
        ((38, Nonterminal(String::from("F"))), 39),
        ((15, Nonterminal(String::from("F"))), 21),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 23,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
