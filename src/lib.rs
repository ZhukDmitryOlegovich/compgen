mod tests;

const GRAMMAR_AXIOM_NAME: &str = "ROOT";

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::format,
    hash::Hash,
    ops::Index,
};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Token<T> {
    tag: TerminalOrFinish,
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

impl ToString for Rule {
    fn to_string(&self) -> String {
        let mut result = format!("{} -> ", self.left.0);
        for term in &self.right {
            result += term.to_string().as_ref();
        }
        result
    }
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
    Shift(i32),
    Accept,
}

impl ToString for LR1Action {
    fn to_string(&self) -> String {
        match self {
            Self::Reduce(rule) => format!("Reduce({})", rule.to_string()),
            Self::Shift(state) => format!("Shift({})", state),
            Self::Accept => String::from("Accept"),
        }
    }
}

struct ParseTables {
    start: i32,
    action: HashMap<(i32, TerminalOrFinish), LR1Action>,
    goto: HashMap<(i32, Nonterminal), i32>,
}

impl ParseTables {
    fn from_automaton(automaton: &DetermenisticLR1Automaton) -> ParseTables {
        let mut cur = 0;
        let mut ids: HashMap<&BTreeSet<LR1Item>, i32> = HashMap::new();
        for (item, _) in &automaton.edges {
            ids.insert(item, cur);
            cur += 1;
        }
        let mut res = ParseTables {
            start: ids[&automaton.start],
            action: HashMap::new(),
            goto: HashMap::new(),
        };
        let mut visited = HashSet::new();
        Self::from_automaton_rec(&automaton.start, &ids, &mut visited, automaton, &mut res);
        res
    }

    fn from_automaton_rec(
        cur: &BTreeSet<LR1Item>,
        ids: &HashMap<&BTreeSet<LR1Item>, i32>,
        visited: &mut HashSet<i32>,
        automaton: &DetermenisticLR1Automaton,
        res: &mut ParseTables,
    ) {
        let id = ids[cur];
        if visited.contains(&id) {
            return;
        }
        visited.insert(id);
        for (other, term) in &automaton.edges[cur] {
            let other_id = ids[other];
            match term {
                Term::Nonterminal(term) => {
                    res.goto.insert((id, term.clone()), other_id);
                }
                Term::Terminal(term) => {
                    res.action.insert(
                        (id, TerminalOrFinish::Terminal(term.clone())),
                        LR1Action::Shift(other_id),
                    );
                }
            }
            Self::from_automaton_rec(other, ids, visited, automaton, res);
        }
        for item in cur {
            if item.is_finish() {
                if item.rule.left.0 == GRAMMAR_AXIOM_NAME {
                    res.action
                        .insert((id, TerminalOrFinish::Finish), LR1Action::Accept);
                } else {
                    res.action.insert(
                        (id, item.lookup.clone()),
                        LR1Action::Reduce(item.rule.clone()),
                    );
                }
            }
        }
    }

    fn print(&self) {
        println!("Start: {}", self.start);
        println!("\nAction:");
        for ((state, term), action) in &self.action {
            println!("({}x{})->{}", state, term.to_string(), action.to_string());
        }
        println!("\nGoto:");
        for ((state, term), new_state) in &self.goto {
            println!("({}x{})->{}", state, term.0, new_state);
        }
    }
}

enum ParseTree<T> {
    Internal(Nonterminal, Vec<ParseTree<T>>),
    Leaf(Token<T>),
}

impl<T: Clone> ParseTree<T> {
    fn from_tables_and_tokens(tables: &ParseTables, tokens: &[Token<T>]) -> Option<ParseTree<T>> {
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

    fn to_graphviz(&self) -> String {
        let mut counter = 0;
        let inner = self.to_graphviz_rec(&mut counter);
        let mut res = String::new();
        res += "digraph G {\n";
        res += inner.as_ref();
        res += "}\n";
        res
    }

    fn to_graphviz_rec(&self, counter: &mut i32) -> String {
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

struct Lexer<'a> {
    cur: Coord,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            cur: Coord {
                line: 1,
                column: 1,
                index: 0,
            },
            input,
        }
    }

    fn get_tokens(&mut self) -> Option<Vec<Token<TokenAttribute>>> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token()?;
            let is_finish = token.tag == TerminalOrFinish::Finish;
            res.push(token);
            if is_finish {
                break;
            }
        }
        Some(res)
    }

    fn get_next_token(&mut self) -> Option<Token<TokenAttribute>> {
        self.skip_spaces();
        let begin = self.cur.clone();
        match self.peek() {
            Some(ch) => {
                if ch.is_uppercase() {
                    let res = self.read_while(|c| !c.is_whitespace() && c != '<' && c != '>');
                    Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::Nonterminal(res),
                        },
                    })
                } else if ch == '<' {
                    self.next();
                    Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("open"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::None,
                        },
                    })
                } else if ch == '>' {
                    self.next();
                    Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("close"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::None,
                        },
                    })
                } else if ch == '\'' {
                    self.read_while(|c| c != '\n');
                    self.next();
                    self.get_next_token()
                } else {
                    let res = self.read_while(|c| !c.is_whitespace() && c != '<' && c != '>');
                    let tag_name = if res == "axiom" { "axiom" } else { "term" };
                    Some(Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from(tag_name))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::Terminal(res),
                        },
                    })
                }
            }
            None => Some(Token {
                tag: TerminalOrFinish::Finish,
                attribute: TokenAttribute {
                    fragment: Fragment {
                        begin: self.cur.clone(),
                        end: self.cur.clone(),
                    },
                    domain_attribute: TokenDomainAttribute::None,
                },
            }),
        }
    }

    fn read_while<F>(&mut self, p: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        loop {
            match self.peek() {
                Some(c) => {
                    if !p(c) {
                        break;
                    }
                    res += c.to_string().as_ref();
                }
                None => break,
            }
            self.next();
        }
        res
    }

    fn skip_spaces(&mut self) {
        while self.is_space() {
            self.next()
        }
    }

    fn is_space(&self) -> bool {
        match self.peek() {
            Some(c) => c.is_whitespace(),
            None => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cur.index as usize)
    }

    fn next(&mut self) {
        match self.peek() {
            Some(t) => {
                if t == '\n' {
                    self.cur.line += 1;
                    self.cur.column = 1;
                } else {
                    self.cur.column += 1;
                }
            }
            _ => (),
        }
        self.cur.index += 1;
    }
}

#[derive(PartialEq, Eq, Debug)]
struct TokenAttribute {
    fragment: Fragment,
    domain_attribute: TokenDomainAttribute,
}

#[derive(PartialEq, Eq, Debug)]
enum TokenDomainAttribute {
    Nonterminal(String),
    Terminal(String),
    None,
}

#[derive(PartialEq, Eq, Debug)]
struct Fragment {
    begin: Coord,
    end: Coord,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Coord {
    line: i32,
    column: i32,
    index: i32,
}
