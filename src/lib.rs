mod tests;

const GRAMMAR_AXIOM_NAME: &str = "ROOT";

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

struct Token<T> {
    tag: String,
    attribute: T,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
struct Nonterminal(String);
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
struct Terminal(String);

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
struct Rule {
    left: Nonterminal,
    right: Vec<Term>,
}

struct Grammar {
    rules: Vec<Rule>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
struct LR1Item {
    rule: Rule,
    position: u32,
    lookup: TerminalOrFinish,
}

impl LR1Item {
    fn is_finish(&self) -> bool {
        self.position == self.rule.right.len() as u32
    }
}

impl ToString for LR1Item {
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

struct NonDeterministicLR1Automaton {
    edges: HashMap<LR1Item, HashMap<LR1Item, TermOrEmpty>>,
    start: LR1Item,
}

impl NonDeterministicLR1Automaton {
    fn from_grammar(grammar: &Grammar) -> NonDeterministicLR1Automaton {
        let mut by_left: HashMap<Nonterminal, Vec<Rule>> = HashMap::new();
        for rule in &grammar.rules {
            match by_left.get_mut(&rule.left) {
                Some(v) => {
                    v.push(rule.clone());
                }
                None => {
                    by_left.insert(rule.left.clone(), vec![rule.clone()]);
                }
            }
        }
        let mut edges = HashMap::new();
        let first = calculate_first(grammar);
        let start = &LR1Item {
            rule: by_left[&Nonterminal(String::from(GRAMMAR_AXIOM_NAME))][0].clone(),
            position: 0,
            lookup: TerminalOrFinish::Finish,
        };
        Self::from_grammar_rec(start, &mut edges, &by_left, &first);
        NonDeterministicLR1Automaton {
            edges,
            start: start.clone(),
        }
    }

    fn from_grammar_rec(
        cur: &LR1Item,
        edges: &mut HashMap<LR1Item, HashMap<LR1Item, TermOrEmpty>>,
        by_left: &HashMap<Nonterminal, Vec<Rule>>,
        first: &HashMap<Nonterminal, HashSet<TerminalOrEmpty>>,
    ) {
        if edges.contains_key(cur) {
            return;
        }
        edges.insert(cur.clone(), HashMap::new());
        if !cur.is_finish() {
            let term = &cur.rule.right[cur.position as usize];
            let next = LR1Item {
                rule: cur.rule.clone(),
                position: cur.position + 1,
                lookup: cur.lookup.clone(),
            };
            let fst = edges.get_mut(cur).expect("no first set");
            fst.insert(next.clone(), TermOrEmpty::Term(term.clone()));
            Self::from_grammar_rec(&next, edges, by_left, first);
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
                            lookups = [a].into_iter().collect();
                            is_nullable = false;
                        }
                    }
                    for lookup in &lookups {
                        for rule in &by_left[nterm] {
                            let next = LR1Item {
                                rule: rule.clone(),
                                position: 0,
                                lookup: lookup.clone(),
                            };
                            let fst = edges.get_mut(cur).expect("no first set");
                            fst.insert(next.clone(), TermOrEmpty::Empty);
                            Self::from_grammar_rec(&next, edges, by_left, first);
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
        result += "fake [style=\"invis\"]\n";
        result += format!("fake -> {}\n", ids[&self.start]).as_ref();
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

impl NonDeterministicLR1Automaton {
    fn get_transitions(&self, vertices: &BTreeSet<LR1Item>) -> HashMap<Term, BTreeSet<LR1Item>> {
        let mut res: HashMap<Term, BTreeSet<LR1Item>> = HashMap::new();
        for vertex in vertices {
            for (other, term) in &self.edges[vertex] {
                if let TermOrEmpty::Term(t) = term {
                    if !res.contains_key(t) {
                        res.insert(t.clone(), BTreeSet::new());
                    }
                    let fst = res.get_mut(t).expect("no set for vertex");
                    fst.insert(other.clone());
                }
            }
        }
        res.into_iter()
            .map(|(t, s)| (t, self.get_epsilon_closure(&s)))
            .collect()
    }

    fn get_epsilon_closure(&self, vertices: &BTreeSet<LR1Item>) -> BTreeSet<LR1Item> {
        let mut res = BTreeSet::new();
        for vertex in vertices {
            if !res.contains(vertex) {
                self.get_epsilon_closure_rec(vertex, &mut res);
            }
        }
        res
    }

    fn get_epsilon_closure_rec(&self, vertex: &LR1Item, res: &mut BTreeSet<LR1Item>) {
        res.insert(vertex.clone());
        for (other, term) in &self.edges[vertex] {
            if let TermOrEmpty::Empty = term {
                if !res.contains(other) {
                    self.get_epsilon_closure_rec(other, res);
                }
            }
        }
    }
}

struct DetermenisticLR1Automaton {
    edges: HashMap<BTreeSet<LR1Item>, HashMap<BTreeSet<LR1Item>, Term>>,
    start: BTreeSet<LR1Item>,
}

impl DetermenisticLR1Automaton {
    fn from_non_deterministic(
        automaton: &NonDeterministicLR1Automaton,
    ) -> DetermenisticLR1Automaton {
        let mut edges = HashMap::new();
        let mut start: BTreeSet<LR1Item> = [automaton.start.clone()].into_iter().collect();
        start = automaton.get_epsilon_closure(&start);
        Self::from_non_deterministic_rec(&start, automaton, &mut edges);
        DetermenisticLR1Automaton { edges, start }
    }

    fn from_non_deterministic_rec(
        cur: &BTreeSet<LR1Item>,
        automaton: &NonDeterministicLR1Automaton,
        edges: &mut HashMap<BTreeSet<LR1Item>, HashMap<BTreeSet<LR1Item>, Term>>,
    ) {
        edges.insert(cur.clone(), HashMap::new());
        for (term, other) in &automaton.get_transitions(cur) {
            let edges_ref = edges.get_mut(cur).expect("state is absent from map");
            edges_ref.insert(other.clone(), term.clone());
            if !edges.contains_key(other) {
                Self::from_non_deterministic_rec(other, automaton, edges);
            }
        }
    }

    fn to_graphviz(&self) -> String {
        let mut result = String::from("digraph G {\nrankdir=\"LR\"\n");
        let mut ids: HashMap<&BTreeSet<LR1Item>, i32> = HashMap::new();
        let mut cur = 0;
        for (items, _) in &self.edges {
            let end = items.iter().find(|x| x.is_finish());
            let color = match end {
                Some(_) => "red",
                None => "black",
            };
            result += format!(
                "{} [shape=\"rectangle\",label=\"{}\", color=\"{}\"]\n",
                cur,
                Self::node_to_graphviz(items),
                color
            )
            .as_ref();
            ids.insert(items, cur);
            cur += 1;
        }
        result += "fake [style=\"invis\"]\n";
        result += format!("fake -> {}\n", ids[&self.start]).as_ref();
        for (items, adjacent) in &self.edges {
            for (other_items, term) in adjacent {
                let id1 = ids[items];
                let id2 = ids[other_items];
                result += format!("{} -> {} [label=\"{}\"]\n", id1, id2, term.to_string()).as_ref();
            }
        }
        result += "}\n";
        result
    }

    fn node_to_graphviz(items: &BTreeSet<LR1Item>) -> String {
        items
            .iter()
            .fold(String::new(), |x, y| x + y.to_string().as_ref() + "\\n")
    }
}

enum LR1Action {
    Reduce(Rule),
    Shift(LR1Item),
    Accept,
}

struct ParseTables {
    action: HashMap<(LR1Item, TerminalOrFinish), LR1Action>,
    goto: HashMap<(LR1Item, Nonterminal), LR1Item>,
}

impl ParseTables {
    fn from_automaton(automaton: &DetermenisticLR1Automaton) -> ParseTables {
        panic!("not implemented");
    }
}

enum ParseTree<T> {
    Internal(Nonterminal, Vec<ParseTree<T>>),
    Leaf(Token<T>),
}

impl<T> ParseTree<T> {
    fn from_tables_and_tokens(tables: &ParseTables, tokens: &[Token<T>]) -> ParseTree<T> {
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
