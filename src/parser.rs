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
                21,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(21),
        ),
        (
            (
                23,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(21),
        ),
        (
            (
                21,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (
                7,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(23),
        ),
        (
            (
                16,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(13),
        ),
        (
            (
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("P")),
                right: vec![],
            }),
        ),
        (
            (19, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                9,
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
                8,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(14),
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
                4,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(16),
        ),
        (
            (
                17,
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
                3,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(23),
        ),
        (
            (
                12,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(12),
        ),
        (
            (
                1,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(4),
        ),
        (
            (11, TerminalOrFinish::Finish),
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
                12,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (
                10,
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
                15,
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
                20,
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
            (17, TerminalOrFinish::Finish),
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
                13,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(9),
        ),
        (
            (
                23,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            }),
        ),
        (
            (0, TerminalOrFinish::Finish),
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
                23,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(12),
        ),
        (
            (
                2,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(18),
        ),
        ((6, TerminalOrFinish::Finish), LR1Action::Accept),
        (
            (9, TerminalOrFinish::Finish),
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
                5,
                TerminalOrFinish::Terminal(Terminal(String::from("close"))),
            ),
            LR1Action::Shift(17),
        ),
        (
            (
                19,
                TerminalOrFinish::Terminal(Terminal(String::from("open"))),
            ),
            LR1Action::Shift(18),
        ),
        (
            (14, TerminalOrFinish::Terminal(Terminal(String::from("ax")))),
            LR1Action::Shift(1),
        ),
        (
            (
                18,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(3),
        ),
        (
            (2, TerminalOrFinish::Finish),
            LR1Action::Reduce(Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            }),
        ),
        (
            (
                21,
                TerminalOrFinish::Terminal(Terminal(String::from("term"))),
            ),
            LR1Action::Shift(12),
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
                12,
                TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
            ),
            LR1Action::Shift(21),
        ),
    ]
    .into_iter()
    .collect();
    let goto = [
        ((8, Nonterminal(String::from("S"))), 6),
        ((7, Nonterminal(String::from("P"))), 15),
        ((23, Nonterminal(String::from("I"))), 22),
        ((2, Nonterminal(String::from("T"))), 19),
        ((21, Nonterminal(String::from("I"))), 20),
        ((19, Nonterminal(String::from("T"))), 19),
        ((12, Nonterminal(String::from("I"))), 10),
        ((19, Nonterminal(String::from("R"))), 0),
        ((8, Nonterminal(String::from("A"))), 2),
        ((2, Nonterminal(String::from("R"))), 11),
        ((3, Nonterminal(String::from("P"))), 5),
    ]
    .into_iter()
    .collect();
    ParseTables {
        start: 8,
        action,
        goto,
    }
}
//@END_PARSE_TABLES@