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
    pub fn from_tables_and_tokens<'a, 'b>(
        tables: &'a ParseTables,
        tokens: &'b [Token<T>],
    ) -> Result<ParseTree<T>, ParseError<T>> {
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
            (26, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (7, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (34, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (28, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (2, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(11),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(12),
        ),
        (
            (35, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(36),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (22, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(36),
        ),
        (
            (3, TerminalOrFinish::Finish),
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
            (10, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(20),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(27),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (17, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(12),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(20),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (11, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(35),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (2, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(12),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (29, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (4, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(39),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(36),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
                right: vec![],
            }),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (4, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(36),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(27),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (3, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (30, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (34, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(22),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(35),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (31, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(39),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(35),
        ),
        (
            (36, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (31, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(35),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(36),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (6, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(11),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(23),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (38, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (0, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        ((14, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(11),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(2),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(22),
        ),
        (
            (30, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (18, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(8),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(8),
        ),
        (
            (19, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (25, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (40, TerminalOrFinish::Finish),
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
            (6, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(35),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (1, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(27),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
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
            (29, TerminalOrFinish::Finish),
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
            (41, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (16, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(20),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (36, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(37),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (13, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
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
            (35, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(0),
        ),
        (
            (31, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (32, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(8),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (19, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (30, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (1, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(22),
        ),
        (
            (18, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(39),
        ),
        (
            (37, TerminalOrFinish::Finish),
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
            (37, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (32, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (5, TerminalOrFinish::Finish),
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
            (18, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("*"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((17, Nonterminal(String::from("F"))), 7),
        ((18, Nonterminal(String::from("T'"))), 26),
        ((10, Nonterminal(String::from("E'"))), 28),
        ((17, Nonterminal(String::from("T"))), 16),
        ((35, Nonterminal(String::from("T"))), 10),
        ((12, Nonterminal(String::from("T"))), 13),
        ((33, Nonterminal(String::from("E'"))), 9),
        ((2, Nonterminal(String::from("E"))), 21),
        ((2, Nonterminal(String::from("T"))), 10),
        ((15, Nonterminal(String::from("F"))), 19),
        ((15, Nonterminal(String::from("E"))), 14),
        ((34, Nonterminal(String::from("T'"))), 3),
        ((20, Nonterminal(String::from("T"))), 33),
        ((11, Nonterminal(String::from("F"))), 19),
        ((32, Nonterminal(String::from("T'"))), 29),
        ((39, Nonterminal(String::from("F"))), 18),
        ((2, Nonterminal(String::from("F"))), 7),
        ((27, Nonterminal(String::from("F"))), 34),
        ((6, Nonterminal(String::from("E'"))), 38),
        ((25, Nonterminal(String::from("E'"))), 5),
        ((4, Nonterminal(String::from("T'"))), 30),
        ((8, Nonterminal(String::from("F"))), 4),
        ((13, Nonterminal(String::from("E'"))), 40),
        ((15, Nonterminal(String::from("T"))), 6),
        ((35, Nonterminal(String::from("F"))), 7),
        ((20, Nonterminal(String::from("F"))), 7),
        ((19, Nonterminal(String::from("T'"))), 31),
        ((35, Nonterminal(String::from("E"))), 24),
        ((16, Nonterminal(String::from("E'"))), 41),
        ((11, Nonterminal(String::from("T"))), 25),
        ((12, Nonterminal(String::from("F"))), 19),
        ((7, Nonterminal(String::from("T'"))), 1),
        ((22, Nonterminal(String::from("F"))), 32),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 15,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
