use std::collections::HashMap;

struct Token<T> {
    tag: String,
    attribute: T,
}

struct Nonterminal(String);
struct Terminal(String);

enum Term {
    Nonterminal(Nonterminal),
    Terminal(Terminal),
}

enum TermOrEmpty {
    Term(Term),
    Empty,
}

enum TerminalOrFinish {
    Terminal(Terminal),
    Finish,
}

struct Rule {
    left: Nonterminal,
    right: Vec<Term>,
}
struct Grammar {
    rules: Vec<Rule>,
}

struct LR1Item<'a> {
    rule: &'a Rule,
    position: u32,
}

struct NonDeterministicLR1Automaton<'a> {
    edges: HashMap<(LR1Item<'a>, LR1Item<'a>), TermOrEmpty>,
}

impl<'a> NonDeterministicLR1Automaton<'_> {
    fn from_grammar(grammar: &'a Grammar) -> NonDeterministicLR1Automaton<'a> {
        panic!("not implemented");
    }
}

struct DetermenisticLR1Automaton<'a> {
    edges: HashMap<(LR1Item<'a>, LR1Item<'a>), Term>,
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
