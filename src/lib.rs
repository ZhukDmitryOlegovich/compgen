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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TermOrFinish {
    Term(Term),
    Finish,
}

impl TermOrFinish {
    fn from_terminal_or_finish(term: &TerminalOrFinish) -> TermOrFinish {
        match term {
            TerminalOrFinish::Finish => TermOrFinish::Finish,
            TerminalOrFinish::Terminal(t) => TermOrFinish::Term(Term::Terminal(t.clone())),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

impl TerminalOrFinish {
    fn from_terminal_or_empty(term: &TerminalOrEmpty) -> Option<TerminalOrFinish> {
        match term {
            TerminalOrEmpty::Terminal(t) => Some(TerminalOrFinish::Terminal(t.clone())),
            TerminalOrEmpty::Empty => None,
        }
    }

    fn from_term_or_finish(term: &TermOrFinish) -> Option<TerminalOrFinish> {
        match term {
            TermOrFinish::Term(Term::Terminal(t)) => Some(TerminalOrFinish::Terminal(t.clone())),
            TermOrFinish::Finish => Some(TerminalOrFinish::Finish),
            _ => None,
        }
    }
}

impl ToString for TerminalOrFinish {
    fn to_string(&self) -> String {
        match self {
            TerminalOrFinish::Terminal(t) => t.0.clone(),
            TerminalOrFinish::Finish => String::from("$"),
        }
    }
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
        String::from(format!(
            "{} -> {}, {}",
            self.rule.left.0,
            right_str,
            self.lookup.to_string()
        ))
    }
}

struct NonDeterministicLR1Automaton<'a> {
    edges: HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, TermOrEmpty>>,
}

impl NonDeterministicLR1Automaton<'_> {
    fn from_grammar<'a>(grammar: &'a Grammar) -> NonDeterministicLR1Automaton<'a> {
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
        let first = calculate_first(grammar);
        let start = &LR1Item {
            rule: by_left[&Nonterminal(String::from(GRAMMAR_AXIOM_NAME))][0],
            position: 0,
            lookup: TerminalOrFinish::Finish,
        };
        NonDeterministicLR1Automaton::from_grammar_rec(start, &mut edges, &by_left, &first);
        NonDeterministicLR1Automaton { edges }
    }

    fn from_grammar_rec<'a>(
        cur: &LR1Item<'a>,
        edges: &mut HashMap<LR1Item<'a>, HashMap<LR1Item<'a>, TermOrEmpty>>,
        by_left: &HashMap<Nonterminal, Vec<&'a Rule>>,
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
                let mut next_terms: Vec<TermOrFinish> = cur.rule.right
                    [(cur.position + 1) as usize..]
                    .iter()
                    .map(|x| TermOrFinish::Term(x.clone()))
                    .collect();
                next_terms.push(TermOrFinish::from_terminal_or_finish(&cur.lookup));
                for term in next_terms {
                    let lookups: HashSet<TerminalOrFinish>;
                    let is_nullable;
                    match term {
                        TermOrFinish::Term(Term::Nonterminal(nt)) => {
                            lookups = first[&nt]
                                .iter()
                                .filter_map(TerminalOrFinish::from_terminal_or_empty)
                                .collect();
                            is_nullable = first[&nt].contains(&TerminalOrEmpty::Empty);
                        }
                        _ => {
                            let a = TerminalOrFinish::from_term_or_finish(&term)
                                .expect("failed type conversion");
                            lookups = [a].iter().cloned().collect();
                            is_nullable = false;
                        }
                    }
                    for lookup in &lookups {
                        for rule in &by_left[nterm] {
                            let next = LR1Item {
                                rule,
                                position: 0,
                                lookup: lookup.clone(),
                            };
                            let fst = edges.get_mut(cur).expect("no first set");
                            fst.insert(next.clone(), TermOrEmpty::Empty);
                            NonDeterministicLR1Automaton::from_grammar_rec(
                                &next, edges, by_left, first,
                            );
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

type DetermenisticLR1State<'a> = HashSet<LR1Item<'a>>;

struct DetermenisticLR1Automaton<'a> {
    edges: HashMap<DetermenisticLR1State<'a>, HashMap<DetermenisticLR1State<'a>, Term>>,
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
