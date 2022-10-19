#[cfg(test)]
mod tests {
    use std::vec;

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
            axiom: Nonterminal(String::from("S")),
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
        add_fake_axiom(&mut grammar);
        grammar
    }

    fn get_cbs_grammar() -> Grammar {
        // S ->
        // S -> (S)S
        let mut grammar = Grammar {
            axiom: Nonterminal(String::from("S")),
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
        add_fake_axiom(&mut grammar);
        grammar
    }

    #[test]
    fn test_non_deterministic_automaton_cbs_graphviz() {
        let grammar = get_cbs_grammar();
        let automaton = NonDeterministicLR1Automaton::from_grammar(&grammar);
        println!("{}", automaton.to_graphviz());
    }

    #[test]
    fn test_non_deterministic_automaton_arithmetic_graphviz() {
        let grammar = get_arithmetic_grammar();
        let automaton = NonDeterministicLR1Automaton::from_grammar(&grammar);
        println!("{}", automaton.to_graphviz());
    }

    #[test]
    fn test_deterministic_automaton_cbs_graphviz() {
        let grammar = get_cbs_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        println!("{}", dfa.to_graphviz());
    }

    #[test]
    fn test_deterministic_automaton_arithmetic_graphviz() {
        let grammar = get_arithmetic_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        println!("{}", dfa.to_graphviz());
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

    #[test]
    fn test_tables_cbs() {
        let grammar = get_cbs_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);
        tables.print();
    }

    #[test]
    fn test_tables_arithmetic() {
        let grammar = get_arithmetic_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);
        tables.print();
    }

    #[test]
    fn test_parse_cbs() {
        let grammar = get_cbs_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);

        let empty_cbs = strings_to_tokens(&[]);
        let res = ParseTree::from_tables_and_tokens(&tables, &empty_cbs);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let cbs = strings_to_tokens(&["(", ")", "(", "(", ")", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &cbs);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let not_cbs = strings_to_tokens(&["(", ")", "(", "(", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &not_cbs);
        assert!(res.is_none());
    }

    #[test]
    fn test_parse_arithmetic() {
        let grammar = get_arithmetic_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);

        let correct = strings_to_tokens(&["x", "+", "x", "*", "(", "x", "+", "x", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &correct);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let incorrect = strings_to_tokens(&["x", "+", "x", "*", "(", "x", "+", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &incorrect);
        assert!(res.is_none());
    }

    #[test]
    fn test_lexical_analysis() {
        let mut lexer = Lexer::new(
            r#"
                ' аксиома
                < axiom <E > >
                ' правила грамматики
                <E <T E' > >
                <E' <+ T E' >
                <>>
                <T <F T' > >
                <T' <* F T' >
                <>>
                <F <n >
                <( E ) > >
                "#,
        );
        let tokens = lexer.get_tokens();
        assert!(tokens.is_some());
    }

    #[test]
    fn test_parse_meta_grammar() {
        let grammar = get_meta_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        println!("{}", dfa.to_graphviz());
        let tables = ParseTables::from_automaton(&dfa);
        let mut lexer = Lexer::new(
            r#"
                <axiom <S>>
                <S <A R>>
                <A <open ax open nterm close close>>
                <R <T R>
                    <>>
                <T <open nterm P close>>
                <P  <open I close P>
                    <>>
                <I  <term I>
                    <nterm I>
                    <>>
            "#,
        );
        let tokens = lexer.get_tokens().unwrap();
        let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
        println!("{}", tree.to_graphviz());

        let parsed = get_grammar_from_tree(&tree);
        assert_eq!(parsed, Some(grammar));
    }

    #[test]
    fn test_tables_to_literal() {
        let grammar = get_meta_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);
        println!("{}", tables.to_rust_function());
    }

    #[test]
    fn test_tables_to_source() {
        let grammar = get_meta_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa);
        println!("{}", tables.to_rust_source().unwrap());
    }

    #[test]
    fn test_calculator_lexer() {
        let mut lexer = calculator::Lexer::new("(1+2)*3-4/5");
        let tokens = lexer.get_tokens().unwrap();
        println!("{:?}", tokens);
    }

    fn strings_to_tokens(v: &[&str]) -> Vec<Token<()>> {
        v.into_iter()
            .map(|x| Token::<()> {
                tag: TerminalOrFinish::Terminal(Terminal(x.to_string())),
                attribute: (),
            })
            .chain([Token::<()> {
                tag: TerminalOrFinish::Finish,
                attribute: (),
            }])
            .collect()
    }

}
