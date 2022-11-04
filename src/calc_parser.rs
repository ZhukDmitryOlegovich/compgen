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
                ((24, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((18, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((16, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(29)),
((25, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((5, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((29, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((1, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((7, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((37, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(17)),
((11, TerminalOrFinish::Finish), LR1Action::Accept),
((40, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((13, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(15)),
((10, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((28, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Shift(21)),
((3, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((21, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((35, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((25, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((2, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((4, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(12)),
((7, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((30, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((21, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((29, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((16, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(15)),
((32, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((2, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(37)),
((21, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((25, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((13, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((32, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((8, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(37)),
((10, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(15)),
((3, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((33, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((7, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(38)),
((13, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((5, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((32, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((20, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((23, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(17)),
((16, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((18, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((39, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((16, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((13, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((8, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(23)),
((40, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((33, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((41, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((39, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(0)),
((12, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((20, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(5)),
((10, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((12, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((17, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((3, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((19, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((20, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(12)),
((4, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((0, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(3)),
((7, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((38, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(3)),
((41, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((33, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((41, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((35, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((13, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(29)),
((0, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(17)),
((27, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(37)),
((39, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(38)),
((1, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((3, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((27, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(23)),
((6, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((19, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((39, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((19, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((7, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(0)),
((16, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((4, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(5)),
((38, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(17)),
((14, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![Term::Terminal(Terminal(String::from("-"))),
Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((1, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((33, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((36, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(12)),
((21, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((26, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![Term::Terminal(Terminal(String::from("-"))),
Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((10, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(29)),
((36, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Shift(5)),
((36, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((9, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![Term::Terminal(Terminal(String::from("+"))),
Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((37, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(3)),
((2, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Shift(23)),
((23, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(3)),
((8, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((30, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((21, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((39, TerminalOrFinish::Terminal(Terminal(String::from("+")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((10, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![],     
            }
        )),
((15, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(40)),
((34, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Shift(19)),
((35, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("*"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((33, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((25, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Shift(0)),
((19, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),
((17, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((25, TerminalOrFinish::Terminal(Terminal(String::from("*")))), LR1Action::Shift(38)),
((22, TerminalOrFinish::Terminal(Terminal(String::from("(")))), LR1Action::Shift(17)),
((18, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((30, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("T'")),
                right: vec![Term::Terminal(Terminal(String::from("/"))),
Term::Nonterminal(Nonterminal(String::from("F"))),
Term::Nonterminal(Nonterminal(String::from("T'"))),
],     
            }
        )),
((31, TerminalOrFinish::Terminal(Terminal(String::from(")")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![Term::Terminal(Terminal(String::from("+"))),
Term::Nonterminal(Nonterminal(String::from("T"))),
Term::Nonterminal(Nonterminal(String::from("E'"))),
],     
            }
        )),
((15, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(33)),
((22, TerminalOrFinish::Terminal(Terminal(String::from("n")))), LR1Action::Shift(3)),
((27, TerminalOrFinish::Finish), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("E'")),
                right: vec![],     
            }
        )),
((3, TerminalOrFinish::Terminal(Terminal(String::from("-")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("n"))),
],     
            }
        )),
((19, TerminalOrFinish::Terminal(Terminal(String::from("/")))), LR1Action::Reduce(
            Rule {
                left: Nonterminal(String::from("F")),
                right: vec![Term::Terminal(Terminal(String::from("("))),
Term::Nonterminal(Nonterminal(String::from("E"))),
Term::Terminal(Terminal(String::from(")"))),
],     
            }
        )),

            ].into_iter().collect();
            let goto = [
                ((37, Nonterminal(String::from("T"))), 8),
((17, Nonterminal(String::from("T"))), 4),
((5, Nonterminal(String::from("T"))), 20),
((4, Nonterminal(String::from("E'"))), 6),
((27, Nonterminal(String::from("E'"))), 24),
((29, Nonterminal(String::from("F"))), 16),
((15, Nonterminal(String::from("F"))), 13),
((10, Nonterminal(String::from("T'"))), 18),
((36, Nonterminal(String::from("E'"))), 26),
((20, Nonterminal(String::from("E'"))), 31),
((37, Nonterminal(String::from("F"))), 7),
((22, Nonterminal(String::from("T"))), 27),
((25, Nonterminal(String::from("T'"))), 35),
((0, Nonterminal(String::from("F"))), 39),
((38, Nonterminal(String::from("F"))), 25),
((23, Nonterminal(String::from("T"))), 2),
((40, Nonterminal(String::from("F"))), 10),
((2, Nonterminal(String::from("E'"))), 14),
((40, Nonterminal(String::from("T"))), 4),
((17, Nonterminal(String::from("F"))), 10),
((40, Nonterminal(String::from("E"))), 34),
((39, Nonterminal(String::from("T'"))), 32),
((13, Nonterminal(String::from("T'"))), 30),
((12, Nonterminal(String::from("T"))), 36),
((12, Nonterminal(String::from("F"))), 10),
((22, Nonterminal(String::from("E"))), 11),
((8, Nonterminal(String::from("E'"))), 9),
((5, Nonterminal(String::from("F"))), 10),
((23, Nonterminal(String::from("F"))), 7),
((7, Nonterminal(String::from("T'"))), 1),
((17, Nonterminal(String::from("E"))), 28),
((16, Nonterminal(String::from("T'"))), 41),
((22, Nonterminal(String::from("F"))), 7),

            ].into_iter().collect();
            ParseTables {
                start: 22,
                action,
                goto,
            }
        }
        
//@END_PARSE_TABLES@
