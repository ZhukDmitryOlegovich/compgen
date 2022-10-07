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

struct LR1Item {
    rule: Rule,
    position: u32,
}

struct NonDeterministicLR1Automaton {
    edges: HashMap<(LR1Item, LR1Item), TermOrEmpty>,
}

impl NonDeterministicLR1Automaton {
    fn from_grammar(grammar: &Grammar) -> NonDeterministicLR1Automaton {
        panic!("not implemented");
    }
}

struct DetermenisticLR1Automaton {
    edges: HashMap<(LR1Item, LR1Item), Term>
}

impl DetermenisticLR1Automaton {
    fn from_non_deterministic(automaton: &NonDeterministicLR1Automaton) -> DetermenisticLR1Automaton {
        panic!("not implemented");
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

impl <T> ParseTree<T> {
    fn from_tables_and_tokens(tables: &ParseTables, tokens: &[Token<T>]) -> ParseTree<T> {
        panic!("not implemented");
    }
}
