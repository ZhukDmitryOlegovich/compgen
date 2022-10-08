#[cfg(test)]
mod tests {
    use crate::*;

    fn get_arithmetic_grammar() -> Grammar {
        // S  -> M Topt
        // Topt -> + M Topt
        // Topt -> EPS
        // M  -> N Mopt
        // Mopt -> * N Mopt
        // Mopt -> EPS
        // N  -> x
        // N  -> ( S )
        let mut grammar = Grammar {
            rules: vec![
                // S  -> M Topt
                Rule {
                    left: Nonterminal(String::from("S")),
                    right: vec![
                        Term::Nonterminal(Nonterminal(String::from("M"))),
                        Term::Nonterminal(Nonterminal(String::from("Topt"))),
                    ],
                },
                // Topt -> + M Topt
                Rule {
                    left: Nonterminal(String::from("Topt")),
                    right: vec![
                        Term::Terminal(Terminal(String::from("+"))),
                        Term::Nonterminal(Nonterminal(String::from("M"))),
                        Term::Nonterminal(Nonterminal(String::from("Topt"))),
                    ],
                },
                // Topt -> EPS
                Rule {
                    left: Nonterminal(String::from("Topt")),
                    right: vec![],
                },
                // M  -> N Mopt
                Rule {
                    left: Nonterminal(String::from("M")),
                    right: vec![
                        Term::Nonterminal(Nonterminal(String::from("N"))),
                        Term::Nonterminal(Nonterminal(String::from("Mopt"))),
                    ],
                },
                // Mopt -> * N Mopt
                Rule {
                    left: Nonterminal(String::from("Mopt")),
                    right: vec![
                        Term::Terminal(Terminal(String::from("*"))),
                        Term::Nonterminal(Nonterminal(String::from("N"))),
                        Term::Nonterminal(Nonterminal(String::from("Mopt"))),
                    ],
                },
                // Mopt -> EPS
                Rule {
                    left: Nonterminal(String::from("Mopt")),
                    right: vec![],
                },
                // N  -> x
                Rule {
                    left: Nonterminal(String::from("N")),
                    right: vec![Term::Terminal(Terminal(String::from("x")))],
                },
                // N  -> ( S )
                Rule {
                    left: Nonterminal(String::from("N")),
                    right: vec![
                        Term::Terminal(Terminal(String::from("("))),
                        Term::Nonterminal(Nonterminal(String::from("S"))),
                        Term::Terminal(Terminal(String::from(")"))),
                    ],
                },
            ],
        };
        add_fake_axiom(&mut grammar, "S");
        grammar
    }

    fn get_cbs_grammar() -> Grammar {
        // S ->
        // S -> (S)S
        let mut grammar = Grammar {
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
        add_fake_axiom(&mut grammar, "S");
        grammar
    }

    #[test]
    fn test_automaton_cbs_graphviz() {
        let grammar = get_cbs_grammar();
        let automaton = NonDeterministicLR1Automaton::from_grammar(&grammar);
        println!("{}", automaton.to_graphviz());
    }

    #[test]
    fn test_automaton_arithmetic_graphviz() {
        let grammar = get_arithmetic_grammar();
        let automaton = NonDeterministicLR1Automaton::from_grammar(&grammar);
        println!("{}", automaton.to_graphviz());
    }

    #[test]
    fn test_first_cbs() {
        let grammar = get_cbs_grammar();
        let first = calculate_first(&grammar);
        assert_eq!(
            first,
            [
                (
                    Nonterminal(String::from(GRAMMAR_AXIOM_NAME)),
                    [
                        TerminalOrEmpty::Empty,
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("S")),
                    [
                        TerminalOrEmpty::Empty,
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                    ]
                    .into_iter()
                    .collect()
                )
            ]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn test_first_arithmetic() {
        let grammar = get_arithmetic_grammar();
        let first = calculate_first(&grammar);
        assert_eq!(
            first,
            [
                (
                    Nonterminal(String::from(GRAMMAR_AXIOM_NAME)),
                    [
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                        TerminalOrEmpty::Terminal(Terminal(String::from("x"))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("S")),
                    [
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                        TerminalOrEmpty::Terminal(Terminal(String::from("x"))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("Topt")),
                    [
                        TerminalOrEmpty::Empty,
                        TerminalOrEmpty::Terminal(Terminal(String::from("+"))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("M")),
                    [
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                        TerminalOrEmpty::Terminal(Terminal(String::from("x"))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("Mopt")),
                    [
                        TerminalOrEmpty::Empty,
                        TerminalOrEmpty::Terminal(Terminal(String::from("*"))),
                    ]
                    .into_iter()
                    .collect()
                ),
                (
                    Nonterminal(String::from("N")),
                    [
                        TerminalOrEmpty::Terminal(Terminal(String::from("("))),
                        TerminalOrEmpty::Terminal(Terminal(String::from("x"))),
                    ]
                    .into_iter()
                    .collect()
                ),
            ]
            .into_iter()
            .collect()
        );
    }
}
