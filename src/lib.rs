use std::{collections::HashMap, fmt::format, hash::Hash};

struct Token<T> {
    tag: String,
    attribute: T,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Nonterminal(String);
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Terminal(String);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Term {
    Nonterminal(Nonterminal),
    Terminal(Terminal),
}

impl ToString for Term {
    fn to_string(&self) -> String {
        match self {
            Term::Nonterminal(Nonterminal(s)) => s.clone(),
            Term::Terminal(Terminal(s)) => s.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TermOrEmpty {
    Term(Term),
    Empty,
}

enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Rule {
    left: Nonterminal,
    right: Vec<Term>,
}

struct Grammar {
    rules: Vec<Rule>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct LR1Item<'a> {
    rule: &'a Rule,
    position: u32,
}

impl LR1Item<'_> {
    fn is_finish(&self) -> bool {
        self.position == self.rule.right.len() as u32
    }
}

impl ToString for LR1Item<'_> {
    fn to_string(&self) -> String {
        let mut right_str = String::new();
        let mut i = 0;
        for term in &self.rule.right {
            if i == self.position {
                right_str.push('*');
            }
            right_str.push_str(term.to_string().as_str());
            i += 1;
        }
        if i == self.position {
            right_str.push('*');
        }
        String::from(format!("{} -> {}", self.rule.left.0, right_str))
    }
}

struct NonDeterministicLR1Automaton<'a> {
    edges: HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, TermOrEmpty>>,
}

impl<'a> NonDeterministicLR1Automaton<'_> {
    fn from_grammar(grammar: &'a Grammar) -> NonDeterministicLR1Automaton<'a> {
        let mut by_left: HashMap<Nonterminal, Vec<&Rule>> = HashMap::new();
        for rule in &grammar.rules {
            match by_left.get_mut(&rule.left) {
                Some(v) => {
                    v.push(rule);
                }
                None => {
                    by_left.insert(rule.left.clone(), vec![rule]);
                }
            }
        }
        let mut edges = HashMap::new();
        for rule in &grammar.rules {
            for position in 0..rule.right.len() as u32 + 1 {
                let item = LR1Item { rule, position };
                let mut adjacent = HashMap::new();
                if position < rule.right.len() as u32 {
                    let next = LR1Item {
                        rule,
                        position: position + 1,
                    };
                    let term = &rule.right[position as usize];
                    adjacent.insert(next, TermOrEmpty::Term(term.clone()));
                    if let Term::Nonterminal(nt) = term {
                        for next_rule in &by_left[nt] {
                            let next = LR1Item {
                                rule: next_rule,
                                position: 0,
                            };
                            adjacent.insert(next, TermOrEmpty::Empty);
                        }
                    }
                }
                edges.insert(item, adjacent);
            }
        }
        NonDeterministicLR1Automaton { edges }
    }

    fn to_graphviz(self: &Self) -> String {
        let mut result = String::from("digraph G {\nrankdir=\"LR\"\n");
        let mut ids: HashMap<&LR1Item, i32> = HashMap::new();
        let mut cur = 0;
        for (item, _) in &self.edges {
            let color = match item.is_finish() {
                true => "red",
                false => "black",
            };
            result += format!(r#"{} [label="{}", shape="rectangle", color="{}"]"#, cur, item.to_string(), color).as_str();
            result += "\n";
            ids.insert(item, cur);
            cur += 1;
        }
        for (item, adjacent) in &self.edges {
            for (other_item, term) in adjacent {
                let id1 = ids[item];
                let id2 = ids[other_item];
                let term_str = match term {
                    TermOrEmpty::Term(t) => t.to_string(),
                    TermOrEmpty::Empty => String::from("EPS"),
                };
                result += format!(r#"{id1} -> {id2} [label="{term_str}"]"#).as_str();
                result += "\n";
            }
        } 
        result += "}\n";
        result
    }
}

struct DetermenisticLR1Automaton<'a> {
    edges: HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, Term>>,
}

impl<'a> DetermenisticLR1Automaton<'_> {
    fn from_non_deterministic(
        automaton: &'a NonDeterministicLR1Automaton,
    ) -> DetermenisticLR1Automaton<'a> {
        panic!("not implemented");
    }
}

enum LR1Action<'a> {
    Reduce(&'a Rule),
    Shift(LR1Item<'a>),
    Accept,
}

struct ParseTables<'a> {
    action: HashMap<(LR1Item<'a>, TerminalOrFinish), LR1Action<'a>>,
    goto: HashMap<(LR1Item<'a>, Nonterminal), LR1Item<'a>>,
}

impl<'a> ParseTables<'_> {
    fn from_automaton(automaton: &'a DetermenisticLR1Automaton) -> ParseTables<'a> {
        panic!("not implemented");
    }
}

enum ParseTree<'a, T> {
    Internal(Nonterminal, Vec<ParseTree<'a, T>>),
    Leaf(&'a Token<T>),
}

impl<'a, 'b, T> ParseTree<'_, T> {
    fn from_tables_and_tokens(tables: &'a ParseTables, tokens: &'b [Token<T>]) -> ParseTree<'b, T> {
        panic!("not implemented");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn graphviz() {
        let grammar = Grammar {
            rules: vec![
                Rule {
                    left: Nonterminal(String::from("S")),
                    right: vec![],
                },
                Rule {
                    left: Nonterminal(String::from("S")),
                    right: vec![
                        Term::Terminal(Terminal(String::from("("))),
                        Term::Nonterminal(Nonterminal(String::from("S"))),
                        Term::Terminal(Terminal(String::from(")"))),
                        Term::Nonterminal(Nonterminal(String::from("S"))),
                    ],
                },
            ],
        };
        let automaton = NonDeterministicLR1Automaton::from_grammar(&grammar);
        println!("{}", automaton.to_graphviz());
    }
}