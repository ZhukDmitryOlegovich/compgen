pub mod parser;
#[cfg(test)]
mod tests;

use crate::parser::*;

const GRAMMAR_AXIOM_NAME: &str = "ROOT";

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

#[derive(Debug)]
pub enum GeneratorError {
    ParseError(ParseError<TokenAttribute>),
    UndeclaredNonterminal(Nonterminal),
    ShiftReduceConflict,
    ReduceReduceConflict,
}

impl Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            GeneratorError::ParseError(err) => {
                let begin = &err.token.attribute.fragment.begin;
                let end = &err.token.attribute.fragment.end;
                let name = match &err.token.tag {
                    TerminalOrFinish::Terminal(t) => &t.0,
                    TerminalOrFinish::Finish => "EOF",
                };
                format!(
                    "Unexpected token {} at {}:{}-{}:{}",
                    name, begin.line, begin.column, end.line, end.column,
                )
            }
            GeneratorError::ShiftReduceConflict => {
                String::from("Encountered shift-reduce conflict while generating tables")
            }
            GeneratorError::ReduceReduceConflict => {
                String::from("Encountered reduce-reduce conflict while generating tables")
            }
            GeneratorError::UndeclaredNonterminal(nterm) => {
                format!("Use of undeclared nonterminal: {}", nterm.0)
            }
        };
        f.write_str(&res)
    }
}

impl From<ParseError<TokenAttribute>> for GeneratorError {
    fn from(err: ParseError<TokenAttribute>) -> Self {
        GeneratorError::ParseError(err)
    }
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TerminalOrEmpty {
    Terminal(Terminal),
    Empty,
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

impl Rule {
    fn to_literal(&self) -> String {
        let nterm = format!("Nonterminal(String::from(\"{}\"))", self.left.0);
        let mut right = String::new();
        for term in &self.right {
            right += match term {
                Term::Nonterminal(nterm) => format!(
                    "Term::Nonterminal(Nonterminal(String::from(\"{}\")))",
                    nterm.0
                ),
                Term::Terminal(term) => {
                    format!("Term::Terminal(Terminal(String::from(\"{}\")))", term.0)
                }
            }
            .as_ref();
            right += ",\n";
        }
        format!(
            r#"
            Rule {{
                left: {},
                right: vec![{}],     
            }}
        "#,
            nterm, right
        )
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grammar {
    axiom: Nonterminal,
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
struct LR0Item {
    rule: Rule,
    position: u32,
}

impl LR0Item {
    fn from_lr1_item(item: &LR1Item) -> LR0Item {
        LR0Item {
            rule: item.rule.clone(),
            position: item.position,
        }
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
            right_str.push(' ');
            i += 1;
        }
        if i == self.position {
            right_str.push('^');
        }
        format!(
            "{} -> {}, {}",
            self.rule.left.0,
            right_str,
            self.lookup.to_string()
        )
    }
}

pub struct NonDeterministicLR1Automaton {
    edges: HashMap<LR1Item, HashMap<LR1Item, TermOrEmpty>>,
    start: LR1Item,
}

impl NonDeterministicLR1Automaton {
    pub fn from_grammar(grammar: &Grammar) -> NonDeterministicLR1Automaton {
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

pub struct DetermenisticLR1Automaton {
    edges: HashMap<BTreeSet<LR1Item>, HashMap<BTreeSet<LR1Item>, Term>>,
    start: BTreeSet<LR1Item>,
}

impl DetermenisticLR1Automaton {
    pub fn from_non_deterministic(
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

fn try_add_action(
    tables: &mut ParseTables,
    state: i32,
    term: TerminalOrFinish,
    action: LR1Action,
) -> Result<(), GeneratorError> {
    if let Some(other_action) = tables.action.get(&(state, term.clone())) {
        if *other_action != action {
            let actions = [&action, other_action];
            for action in actions {
                if let LR1Action::Shift(_) = action {
                    return Err(GeneratorError::ShiftReduceConflict);
                }
            }
            return Err(GeneratorError::ReduceReduceConflict);
        }
    };
    tables.action.insert((state, term), action);
    Ok(())
}

pub enum ParseTablesType {
    LR1,
    LALR,
}

impl ParseTables {
    pub fn from_string(
        input: &str,
        tables_type: ParseTablesType,
    ) -> Result<ParseTables, GeneratorError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.get_tokens();
        let tables = parser::get_parse_tables();
        let tree = ParseTree::from_tables_and_tokens(&tables, &tokens)?;
        let encoded_grammar = get_grammar_from_tree(&tree)?;
        let nfa = NonDeterministicLR1Automaton::from_grammar(&encoded_grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        ParseTables::from_automaton(&dfa, tables_type)
    }

    pub fn from_automaton(
        automaton: &DetermenisticLR1Automaton,
        tables_type: ParseTablesType,
    ) -> Result<ParseTables, GeneratorError> {
        let mut cur = 0;
        let mut ids: HashMap<&BTreeSet<LR1Item>, i32> = HashMap::new();
        let mut lr0_ids: HashMap<BTreeSet<LR0Item>, i32> = HashMap::new();
        for item in automaton.edges.keys() {
            match tables_type {
                ParseTablesType::LR1 => {
                    ids.insert(item, cur);
                    cur += 1;
                }
                ParseTablesType::LALR => {
                    let lr0_kernel: BTreeSet<LR0Item> =
                        item.iter().map(LR0Item::from_lr1_item).collect();
                    if !lr0_ids.contains_key(&lr0_kernel) {
                        lr0_ids.insert(lr0_kernel.clone(), cur);
                        cur += 1;
                    }
                    let id = lr0_ids[&lr0_kernel];
                    ids.insert(item, id);
                }
            }
        }
        let mut res = ParseTables {
            start: ids[&automaton.start],
            action: HashMap::new(),
            goto: HashMap::new(),
        };
        let mut visited = HashSet::new();
        Self::from_automaton_rec(&automaton.start, &ids, &mut visited, automaton, &mut res)?;
        Ok(res)
    }

    fn from_automaton_rec<'a: 'b, 'b, 'c: 'b>(
        cur: &'a BTreeSet<LR1Item>,
        ids: &HashMap<&BTreeSet<LR1Item>, i32>,
        visited: &mut HashSet<&'b BTreeSet<LR1Item>>,
        automaton: &'c DetermenisticLR1Automaton,
        res: &mut ParseTables,
    ) -> Result<(), GeneratorError> {
        let id = ids[cur];
        if visited.contains(cur) {
            return Ok(());
        }
        visited.insert(cur);
        for (other, term) in &automaton.edges[cur] {
            let other_id = ids[other];
            match term {
                Term::Nonterminal(term) => {
                    res.goto.insert((id, term.clone()), other_id);
                }
                Term::Terminal(term) => {
                    try_add_action(
                        res,
                        id,
                        TerminalOrFinish::Terminal(term.clone()),
                        LR1Action::Shift(other_id),
                    )?;
                }
            }
            Self::from_automaton_rec(other, ids, visited, automaton, res)?;
        }
        for item in cur {
            if item.is_finish() {
                if item.rule.left.0 == GRAMMAR_AXIOM_NAME {
                    try_add_action(res, id, TerminalOrFinish::Finish, LR1Action::Accept)
                        .expect("accept can not have conflicts");
                } else {
                    try_add_action(
                        res,
                        id,
                        item.lookup.clone(),
                        LR1Action::Reduce(item.rule.clone()),
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn to_rust_source(&self) -> String {
        let parser_source = include_bytes!("parser.rs");
        let parser_source: Vec<String> = String::from_utf8_lossy(parser_source)
            .lines()
            .map(|x| x.to_string())
            .collect();
        let start_index = parser_source
            .iter()
            .position(|x| x.starts_with("//@START_PARSE_TABLES@"))
            .expect("no @START_PARSE_TABLES@ comment in parser.rs");
        let finish_index = parser_source
            .iter()
            .position(|x| x.starts_with("//@END_PARSE_TABLES@"))
            .expect("no @END_Pno @END_PARSE_TABLES@ comment in parser.rs");
        let tables = self.to_rust_function();
        [
            parser_source[0..=start_index].join("\n"),
            tables,
            parser_source[finish_index..parser_source.len()].join("\n"),
        ]
        .join("\n")
    }

    fn to_rust_function(&self) -> String {
        let mut action_entries = String::new();
        let mut goto_entries = String::new();
        for ((state, term), action) in &self.action {
            let term_str = match term {
                TerminalOrFinish::Terminal(Terminal(s)) => format!(
                    "TerminalOrFinish::Terminal(Terminal(String::from(\"{}\")))",
                    s
                ),
                TerminalOrFinish::Finish => String::from("TerminalOrFinish::Finish"),
            };
            let action_str = match action {
                LR1Action::Shift(state) => format!("LR1Action::Shift({})", state),
                LR1Action::Reduce(rule) => format!("LR1Action::Reduce({})", rule.to_literal()),
                LR1Action::Accept => String::from("LR1Action::Accept"),
            };
            let entry = format!("(({}, {}), {}),\n", state, term_str, action_str);
            action_entries += entry.as_ref();
        }
        for ((cur_state, nterm), next_state) in &self.goto {
            let nterm = format!("Nonterminal(String::from(\"{}\"))", nterm.0);
            let entry = format!("(({}, {}), {}),\n", cur_state, nterm, next_state);
            goto_entries += entry.as_ref();
        }
        format!(
            r#"
        pub fn get_parse_tables() -> ParseTables {{
            let action = [
                {}
            ].into_iter().collect();
            let goto = [
                {}
            ].into_iter().collect();
            ParseTables {{
                start: {},
                action,
                goto,
            }}
        }}
        "#,
            action_entries, goto_entries, self.start
        )
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

fn add_fake_axiom(grammar: &mut Grammar) {
    grammar.rules.push(Rule {
        left: Nonterminal(String::from(GRAMMAR_AXIOM_NAME)),
        right: vec![Term::Nonterminal(grammar.axiom.clone())],
    });
}

pub struct Lexer<'a> {
    cur: Coord,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            cur: Coord {
                line: 1,
                column: 1,
                index: 0,
            },
            input,
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token<TokenAttribute>> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token();
            let is_finish = token.tag == TerminalOrFinish::Finish;
            res.push(token);
            if is_finish {
                break;
            }
        }
        res
    }

    fn get_next_token(&mut self) -> Token<TokenAttribute> {
        self.skip_spaces();
        let begin = self.cur.clone();
        match self.peek() {
            Some(ch) => {
                if ch.is_uppercase() {
                    let res = self.read_while(|c| !c.is_whitespace() && c != '<' && c != '>');
                    Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("nterm"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::Nonterminal(res),
                        },
                    }
                } else if ch == '<' {
                    self.next();
                    Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("open"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::None,
                        },
                    }
                } else if ch == '>' {
                    self.next();
                    Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from("close"))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::None,
                        },
                    }
                } else if ch == '\'' {
                    self.read_while(|c| c != '\n');
                    self.next();
                    self.get_next_token()
                } else {
                    let res = self.read_while(|c| !c.is_whitespace() && c != '<' && c != '>');
                    let tag_name = if res == "axiom" { "ax" } else { "term" };
                    Token {
                        tag: TerminalOrFinish::Terminal(Terminal(String::from(tag_name))),
                        attribute: TokenAttribute {
                            fragment: Fragment {
                                begin,
                                end: self.cur.clone(),
                            },
                            domain_attribute: TokenDomainAttribute::Terminal(res),
                        },
                    }
                }
            }
            None => Token {
                tag: TerminalOrFinish::Finish,
                attribute: TokenAttribute {
                    fragment: Fragment {
                        begin: self.cur.clone(),
                        end: self.cur.clone(),
                    },
                    domain_attribute: TokenDomainAttribute::None,
                },
            },
        }
    }

    fn read_while<F>(&mut self, p: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while let Some(c) = self.peek() {
            if !p(c) {
                break;
            }
            res += c.to_string().as_ref();
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
        if let Some(t) = self.peek() {
            if t == '\n' {
                self.cur.line += 1;
                self.cur.column = 1;
            } else {
                self.cur.column += 1;
            }
        }
        self.cur.index += 1;
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TokenAttribute {
    fragment: Fragment,
    domain_attribute: TokenDomainAttribute,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum TokenDomainAttribute {
    Nonterminal(String),
    Terminal(String),
    None,
}

impl TokenDomainAttribute {
    fn as_nonterminal(&self) -> Option<String> {
        if let TokenDomainAttribute::Nonterminal(s) = self {
            return Some(s.clone());
        }
        None
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Fragment {
    begin: Coord,
    end: Coord,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Coord {
    line: i32,
    column: i32,
    index: i32,
}

pub fn get_grammar_from_tree(root: &ParseTree<TokenAttribute>) -> Result<Grammar, GeneratorError> {
    let (_, root_children) = root.as_internal().unwrap();
    let (_, children) = root_children[0].as_internal().unwrap();
    let t = children[3].as_leaf().unwrap();
    let axiom_name = t.attribute.domain_attribute.as_nonterminal().unwrap();
    let axiom = Nonterminal(axiom_name);
    let rules = get_rules_from_tree(&root_children[1]);
    let mut grammar = Grammar { axiom, rules };
    add_fake_axiom(&mut grammar);
    validate_grammar(&grammar)?;
    Ok(grammar)
}

fn get_rules_from_tree(root: &ParseTree<TokenAttribute>) -> Vec<Rule> {
    let (_, children) = root.as_internal().unwrap();
    if children.is_empty() {
        return Vec::new();
    }
    let right = get_rules_from_tree(&children[1]);
    let (_, children) = &children[0].as_internal().unwrap();
    let t = children[1].as_leaf().unwrap();
    let name = t.attribute.domain_attribute.as_nonterminal().unwrap();
    let left = Nonterminal(name);
    let mut rules = get_subrules_from_tree(&left, &children[2]);
    rules.extend(right);
    rules
}

fn get_subrules_from_tree(left: &Nonterminal, root: &ParseTree<TokenAttribute>) -> Vec<Rule> {
    let (_, children) = root.as_internal().unwrap();
    if children.is_empty() {
        return Vec::new();
    }
    let right = get_subrules_from_tree(left, &children[3]);
    let terms = get_terms_from_subtree(&children[1]);
    let mut res = vec![Rule {
        left: left.clone(),
        right: terms,
    }];
    res.extend(right);
    res
}

fn get_terms_from_subtree(root: &ParseTree<TokenAttribute>) -> Vec<Term> {
    let (_, children) = root.as_internal().unwrap();
    if children.is_empty() {
        return Vec::new();
    }
    let right = get_terms_from_subtree(&children[1]);
    let t = children[0].as_leaf().unwrap();
    match t.attribute.domain_attribute.clone() {
        TokenDomainAttribute::Nonterminal(nterm) => {
            let mut res = vec![Term::Nonterminal(Nonterminal(nterm))];
            res.extend(right);
            res
        }
        TokenDomainAttribute::Terminal(term) => {
            let mut res = vec![Term::Terminal(Terminal(term))];
            res.extend(right);
            res
        }
        _ => panic!("must be terminal or nonterminal"),
    }
}

fn validate_grammar(grammar: &Grammar) -> Result<(), GeneratorError> {
    let left: HashSet<Nonterminal> = grammar.rules.iter().map(|x| x.left.clone()).collect();
    for rule in &grammar.rules {
        for term in &rule.right {
            if let Term::Nonterminal(nterm) = term {
                if !left.contains(nterm) {
                    return Err(GeneratorError::UndeclaredNonterminal(nterm.clone()));
                }
            }
        }
    }
    Ok(())
}
