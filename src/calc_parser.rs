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

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct Nonterminal(pub String);
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]

pub struct Terminal(pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token<T> {
    pub tag: TerminalOrFinish,
    pub attribute: T,
}

pub enum ParseTree<T> {
    Internal(Nonterminal, Vec<ParseTree<T>>),
    Leaf(Token<T>),
}

impl<T: Clone> ParseTree<T> {
    pub fn from_tables_and_tokens(
        tables: &ParseTables,
        tokens: &[Token<T>],
    ) -> Option<ParseTree<T>> {
        let mut states = vec![tables.start];
        let mut trees: Vec<ParseTree<T>> = Vec::new();
        let mut token_index = 0;
        while token_index < tokens.len() {
            let token = &tokens[token_index];
            let cur_state = states.last()?;
            let action = tables.action.get(&(cur_state.clone(), token.tag.clone()))?;
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
                        children.push(trees.pop()?);
                    }
                    children.reverse();
                    trees.push(ParseTree::Internal(rule.left.clone(), children));
                    let cur = states.last()?;
                    let next = tables.goto.get(&(cur.clone(), rule.left.clone()))?;
                    states.push(next.clone());
                }
                LR1Action::Accept => {
                    return trees.pop();
                }
            }
        }
        None
    }
}

//@START_PARSE_TABLES@

pub fn get_parse_tables() -> ParseTables {
    let action = [
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (38, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (41, TerminalOrFinish::Finish),
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
            (1, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(31),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(9),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (23, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
                ],
            }),
        ),
        (
            (3, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(32),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(10),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
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
            (1, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (31, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(9),
        ),
        (
            (5, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
            (12, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (5, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (40, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(10),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (15, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
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
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (3, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(9),
        ),
        (
            (13, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![
                    Term::Terminal(Terminal(String::from("/"))),
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        ((18, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(13),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (7, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (36, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (40, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (2, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("E'"))),
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
            (34, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (28, TerminalOrFinish::Finish),
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
            (0, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
            (35, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(10),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(13),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (35, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
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
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            LR1Action::Shift(35),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(35),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (26, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Shift(37),
        ),
        (
            (41, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (6, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(31),
        ),
        (
            (24, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (9, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (39, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (7, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(3),
        ),
        (
            (22, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(3),
        ),
        (
            (30, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(25),
        ),
        (
            (40, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(3),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(13),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (26, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (31, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(32),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (38, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(29),
        ),
        (
            (33, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (16, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (11, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
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
            (7, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (17, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(32),
        ),
        (
            (4, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(35),
        ),
        (
            (22, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (11, TerminalOrFinish::Finish),
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
            (40, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(32),
        ),
        (
            (15, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(12),
        ),
        (
            (24, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Shift(31),
        ),
        (
            (10, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(9),
        ),
        (
            (32, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (27, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
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
            (5, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
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
            (32, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (0, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("F"))),
                    Term::Nonterminal(Nonterminal(String::from("T'"))),
                ],
            }),
        ),
        (
            (21, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Shift(41),
        ),
        (
            (25, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (13, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (8, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (6, TerminalOrFinish::Terminal(Terminal(String::from("+")))),
            LR1Action::Shift(17),
        ),
        (
            (19, TerminalOrFinish::Finish),
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
            (1, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],
            }),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (37, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (32, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (11, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
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
            (29, TerminalOrFinish::Terminal(Terminal(String::from("/")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (40, TerminalOrFinish::Terminal(Terminal(String::from("-")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],
            }),
        ),
        (
            (20, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(9),
        ),
        (
            (26, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(12),
        ),
        (
            (29, TerminalOrFinish::Terminal(Terminal(String::from(")")))),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n")))],
            }),
        ),
        (
            (12, TerminalOrFinish::Terminal(Terminal(String::from("(")))),
            LR1Action::Shift(38),
        ),
        (
            (34, TerminalOrFinish::Terminal(Terminal(String::from("*")))),
            LR1Action::Shift(12),
        ),
        (
            (36, TerminalOrFinish::Finish),
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
            (10, TerminalOrFinish::Terminal(Terminal(String::from("n")))),
            LR1Action::Shift(32),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((6, Nonterminal(String::from("E'"))), 19),
        ((12, Nonterminal(String::from("F"))), 15),
        ((7, Nonterminal(String::from("T'"))), 11),
        ((39, Nonterminal(String::from("E'"))), 14),
        ((10, Nonterminal(String::from("F"))), 40),
        ((38, Nonterminal(String::from("T"))), 4),
        ((15, Nonterminal(String::from("T'"))), 16),
        ((31, Nonterminal(String::from("T"))), 24),
        ((13, Nonterminal(String::from("T"))), 33),
        ((4, Nonterminal(String::from("E'"))), 23),
        ((24, Nonterminal(String::from("E'"))), 28),
        ((9, Nonterminal(String::from("T"))), 4),
        ((38, Nonterminal(String::from("F"))), 26),
        ((22, Nonterminal(String::from("T'"))), 5),
        ((9, Nonterminal(String::from("E"))), 21),
        ((37, Nonterminal(String::from("F"))), 34),
        ((13, Nonterminal(String::from("F"))), 26),
        ((40, Nonterminal(String::from("T'"))), 36),
        ((38, Nonterminal(String::from("E"))), 30),
        ((20, Nonterminal(String::from("T"))), 1),
        ((9, Nonterminal(String::from("F"))), 26),
        ((34, Nonterminal(String::from("T'"))), 8),
        ((17, Nonterminal(String::from("F"))), 22),
        ((26, Nonterminal(String::from("T'"))), 0),
        ((17, Nonterminal(String::from("T"))), 6),
        ((35, Nonterminal(String::from("T"))), 39),
        ((35, Nonterminal(String::from("F"))), 26),
        ((1, Nonterminal(String::from("E'"))), 2),
        ((33, Nonterminal(String::from("E'"))), 27),
        ((3, Nonterminal(String::from("F"))), 7),
        ((31, Nonterminal(String::from("F"))), 22),
        ((20, Nonterminal(String::from("E"))), 18),
        ((20, Nonterminal(String::from("F"))), 22),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 20,
        action,
        goto,
    }
}

//@END_PARSE_TABLES@
