mod tests;

const GRAMMAR_AXIOM_NAME: &str = "ROOT";

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TerminalOrEmpty {
    Terminal(Terminal),
    Empty,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Rule {
    left: Nonterminal,
    right: Vec<Term>,
}

struct Grammar {
    rules: Vec<Rule>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct LR1Item<'a> {
    rule: &'a Rule,
    position: u32,
    lookup: TerminalOrFinish,
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
                right_str.push('^');
            }
            right_str.push_str(term.to_string().as_str());
            i += 1;
        }
        if i == self.position {
            right_str.push('^');
        }
        String::from(format!("{} -> {}", self.rule.left.0, right_str))
    }
}

struct NonDeterministicLR1Automaton<'a> {
    edges: HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, TermOrEmpty>>,
}

impl NonDeterministicLR1Automaton<'_> {
    fn from_grammar<'a>(grammar: &'a Grammar) -> NonDeterministicLR1Automaton<'a> {
        // let mut by_left: HashMap<Nonterminal, Vec<&Rule>> = HashMap::new();
        // for rule in &grammar.rules {
        //     match by_left.get_mut(&rule.left) {
        //         Some(v) => {
        //             v.push(rule);
        //         }
        //         None => {
        //             by_left.insert(rule.left.clone(), vec![rule]);
        //         }
        //     }
        // }
        let mut edges = HashMap::new();
        // for rule in &grammar.rules {
        //     for position in 0..rule.right.len() as u32 + 1 {
        //         let item = LR1Item { rule, position };
        //         let mut adjacent = HashMap::new();
        //         if position < rule.right.len() as u32 {
        //             let next = LR1Item {
        //                 rule,
        //                 position: position + 1,
        //             };
        //             let term = &rule.right[position as usize];
        //             adjacent.insert(next, TermOrEmpty::Term(term.clone()));
        //             if let Term::Nonterminal(nt) = term {
        //                 for next_rule in &by_left[nt] {
        //                     let next = LR1Item {
        //                         rule: next_rule,
        //                         position: 0,
        //                     };
        //                     adjacent.insert(next, TermOrEmpty::Empty);
        //                 }
        //             }
        //         }
        //         edges.insert(item, adjacent);
        //     }
        // }
        NonDeterministicLR1Automaton { edges }
    }

    fn from_grammar_rec<'a>(
        cur: &LR1Item<'a>,
        edges: &mut HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, TermOrEmpty>>,
        by_left: &'a HashMap<Nonterminal, Vec<&Rule>>,
        first: &HashMap<Nonterminal, HashSet<TerminalOrEmpty>>,
    ) {
        if edges.contains_key(cur) {
            return;
        }
        edges.insert(cur.clone(), HashMap::new());
        if !cur.is_finish() {
            let term = &cur.rule.right[cur.position as usize];
            let next = LR1Item {
                rule: cur.rule,
                position: cur.position + 1,
                lookup: cur.lookup.clone(),
            };
            let fst = edges.get_mut(cur).expect("no first set");
            fst.insert(next.clone(), TermOrEmpty::Term(term.clone()));
            NonDeterministicLR1Automaton::from_grammar_rec(&next, edges, by_left, first);
            if let Term::Nonterminal(nterm) = term {
                for term in &cur.rule.right[(cur.position + 1) as usize..] {
                    let lookups: HashSet<TerminalOrEmpty>;
                    let is_nullable;
                    match term {
                        Term::Terminal(s) => {
                            lookups = [TerminalOrEmpty::Terminal(s.clone())].iter().cloned().collect();
                            is_nullable = false;
                        }
                        Term::Nonterminal(nt) => {
                            lookups = first[nt].iter().filter(|x| {
                                match x {
                                    TerminalOrEmpty::Empty => false,
                                    _ => true,
                                }
                            }).cloned().collect();
                            is_nullable = first[nt].contains(&TerminalOrEmpty::Empty);
                        }
                    } 
                    for lookup in &lookups {
                        let lookup = match lookup {
                            TerminalOrEmpty::Empty => {
                                continue;
                            }
                            TerminalOrEmpty::Terminal(t) => {
                                TerminalOrFinish::Terminal(t.clone())
                            }
                        };
                        for rule in &by_left[nterm] {
                            let next = LR1Item {
                                rule,
                                position: 0,
                                lookup: lookup.clone(),
                            };
                            let fst = edges.get_mut(cur).expect("no first set");
                            fst.insert(next.clone(), TermOrEmpty::Empty);
                            NonDeterministicLR1Automaton::from_grammar_rec(&next, edges, by_left, first);
                        } 
                    }
                    if !is_nullable {
                        break;
                    }
                }
            }
        }
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
            result += format!(
                r#"{} [label="{}", shape="rectangle", color="{}"]"#,
                cur,
                item.to_string(),
                color
            )
            .as_str();
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

impl<T> ParseTree<'_, T> {
    fn from_tables_and_tokens<'a, 'b>(
        tables: &'a ParseTables,
        tokens: &'b [Token<T>],
    ) -> ParseTree<'b, T> {
        panic!("not implemented");
    }
}

fn calculate_first(grammar: &Grammar) -> HashMap<Nonterminal, HashSet<TerminalOrEmpty>> {
    let mut first = HashMap::new();
    let mut nullable: HashSet<Nonterminal> = HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for rule in &grammar.rules {
            if nullable.contains(&rule.left) {
                continue;
            }
            let mut ok = true;
            for term in &rule.right {
                ok = ok
                    && match term {
                        Term::Nonterminal(s) => nullable.contains(s),
                        Term::Terminal(_) => false,
                    };
            }
            if ok {
                nullable.insert(rule.left.clone());
                changed = true;
            }
        }
    }
    for rule in &grammar.rules {
        if first.get(&rule.left).is_none() {
            first.insert(rule.left.clone(), HashSet::new());
        }
        for term in &rule.right {
            if let Term::Nonterminal(term) = term {
                if first.get(term).is_none() {
                    first.insert(term.clone(), HashSet::new());
                }
            }
        }
    }
    for term in &nullable {
        let first_ref = first.get_mut(term).expect("no first set");
        first_ref.insert(TerminalOrEmpty::Empty);
    }
    changed = true;
    while changed {
        changed = false;
        for rule in &grammar.rules {
            for term in &rule.right {
                match term {
                    Term::Terminal(t) => {
                        let val = TerminalOrEmpty::Terminal(t.clone());
                        changed = changed || !first[&rule.left].contains(&val);
                        let fst = first.get_mut(&rule.left).expect("no first set");
                        fst.insert(val);
                        break;
                    }
                    Term::Nonterminal(nterm) => {
                        let mut ns: HashSet<TerminalOrEmpty> =
                            first[&rule.left].union(&first[nterm]).cloned().collect();
                        if !nullable.contains(&rule.left) {
                            ns.remove(&TerminalOrEmpty::Empty);
                        }
                        changed = changed || ns.len() != first[&rule.left].len();
                        let fst = first.get_mut(&rule.left).expect("no first set");
                        *fst = ns;
                        if !nullable.contains(nterm) {
                            break;
                        }
                    }
                }
            }
        }
    }
    first
}

fn add_fake_axiom(grammar: &mut Grammar, current_axiom: &str) {
    grammar.rules.push(Rule {
        left: Nonterminal(String::from(GRAMMAR_AXIOM_NAME)),
        right: vec![Term::Nonterminal(Nonterminal(String::from(current_axiom)))],
    });
}
