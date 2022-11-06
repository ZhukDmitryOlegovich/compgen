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
    let tables = ParseTables::from_automaton(&dfa, ParseTablesType::LR1);
    tables.unwrap().print();
}

#[test]
fn test_tables_arithmetic() {
    let grammar = get_arithmetic_grammar();
    let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    let tables = ParseTables::from_automaton(&dfa, ParseTablesType::LR1);
    tables.unwrap().print();
}

#[test]
fn test_parse_cbs() {
    for method in [ParseTablesType::LR1, ParseTablesType::LALR] {
        let grammar = get_cbs_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa, method).unwrap();

        let empty_cbs = strings_to_tokens(&[]);
        let res = ParseTree::from_tables_and_tokens(&tables, &empty_cbs);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let cbs = strings_to_tokens(&["(", ")", "(", "(", ")", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &cbs);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let not_cbs = strings_to_tokens(&["(", ")", "(", "(", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &not_cbs);
        println!("{:?}", res.expect_err("mskekg"));
    }
}

#[test]
fn test_parse_arithmetic() {
    for method in [ParseTablesType::LR1, ParseTablesType::LALR] {
        let grammar = get_arithmetic_grammar();
        let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
        let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
        let tables = ParseTables::from_automaton(&dfa, method).unwrap();

        let correct = strings_to_tokens(&["x", "+", "x", "*", "(", "x", "+", "x", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &correct);
        println!("{}", res.expect("no parse tree was returned").to_graphviz());

        let incorrect = strings_to_tokens(&["x", "+", "x", "*", "(", "x", "+", ")"]);
        let res = ParseTree::from_tables_and_tokens(&tables, &incorrect);
        assert!(res.is_err());
    }
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
    println!("{:?}", tokens);
}

#[test]
fn test_parse_meta_grammar() {
    let grammar = get_meta_grammar();
    let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    println!("{}", dfa.to_graphviz());
    let tables = ParseTables::from_automaton(&dfa, ParseTablesType::LR1).unwrap();
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
    let tokens = lexer.get_tokens();
    let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
    println!("{}", tree.to_graphviz());

    let parsed = get_grammar_from_tree(&tree).unwrap();
    assert_eq!(parsed, grammar);
}

#[test]
fn test_tables_to_literal() {
    let grammar = get_meta_grammar();
    let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    let tables = ParseTables::from_automaton(&dfa, ParseTablesType::LR1).unwrap();
    println!("{}", tables.to_rust_function());
}

#[test]
fn test_tables_to_source() {
    let grammar = get_meta_grammar();
    let nfa = NonDeterministicLR1Automaton::from_grammar(&grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    let tables = ParseTables::from_automaton(&dfa, ParseTablesType::LR1).unwrap();
    println!("{}", tables.to_rust_source());
}

#[test]
fn test_not_lr1() {
    let input = "
    <axiom <S>>
    <S <>
       <a S a>>";
    let res = ParseTables::from_string(input, ParseTablesType::LR1);
    let err = res.unwrap_err();
    assert!(matches!(err, GeneratorError::ShiftReduceConflict));
}

#[test]
fn test_not_lalr() {
    let input = "
    <axiom <S>>
    <S <a E a>
       <b E b>
       <a F b>
       <b F a>>
    <E <e>>
    <F <e>>";
    let res = ParseTables::from_string(input, ParseTablesType::LALR);
    let err = res.unwrap_err();
    assert!(matches!(err, GeneratorError::ReduceReduceConflict));
    let res = ParseTables::from_string(input, ParseTablesType::LR1);
    assert!(res.is_ok());
}

#[test]
fn test_undeclared_nterm_usage() {
    let input = "
    <axiom <S>>
    <S <A>>";
    let res = ParseTables::from_string(input, ParseTablesType::LR1);
    let err = res.unwrap_err();
    let ok = match err {
        GeneratorError::UndeclaredNonterminal(nterm) => nterm.0 == "A",
        _ => false,
    };
    assert!(ok);
}

fn strings_to_tokens(v: &[&str]) -> Vec<Token<()>> {
    v.iter()
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

fn get_meta_grammar() -> Grammar {
    let mut grammar = Grammar {
        // <axiom <S>>
        // <S <A R>>
        // <A <open ax open nterm close close>>
        // <R <T R>
        //     <>>
        // <T <open nterm P close>>
        // <P  <open I close P>
        //     <>>
        // <I  <term I>
        //     <nterm I>
        //     <>>
        axiom: Nonterminal(String::from("S")),
        rules: vec![
            // <S <A R>>
            Rule {
                left: Nonterminal(String::from("S")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("A"))),
                    Term::Nonterminal(Nonterminal(String::from("R"))),
                ],
            },
            // <A <open ax open nterm close close>>
            Rule {
                left: Nonterminal(String::from("A")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("ax"))),
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Terminal(Terminal(String::from("close"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            },
            // <R <T R>>
            Rule {
                left: Nonterminal(String::from("R")),
                right: vec![
                    Term::Nonterminal(Nonterminal(String::from("T"))),
                    Term::Nonterminal(Nonterminal(String::from("R"))),
                ],
            },
            // <R <>>
            Rule {
                left: Nonterminal(String::from("R")),
                right: vec![],
            },
            // <T <open nterm P close>>
            Rule {
                left: Nonterminal(String::from("T")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Nonterminal(Nonterminal(String::from("P"))),
                    Term::Terminal(Terminal(String::from("close"))),
                ],
            },
            // <P  <open I close P>>
            Rule {
                left: Nonterminal(String::from("P")),
                right: vec![
                    Term::Terminal(Terminal(String::from("open"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                    Term::Terminal(Terminal(String::from("close"))),
                    Term::Nonterminal(Nonterminal(String::from("P"))),
                ],
            },
            // <P <>>
            Rule {
                left: Nonterminal(String::from("P")),
                right: vec![],
            },
            // <I  <term I>>
            Rule {
                left: Nonterminal(String::from("I")),
                right: vec![
                    Term::Terminal(Terminal(String::from("term"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                ],
            },
            // <I  <nterm I>>
            Rule {
                left: Nonterminal(String::from("I")),
                right: vec![
                    Term::Terminal(Terminal(String::from("nterm"))),
                    Term::Nonterminal(Nonterminal(String::from("I"))),
                ],
            },
            // <I  <>>
            Rule {
                left: Nonterminal(String::from("I")),
                right: vec![],
            },
        ],
    };
    add_fake_axiom(&mut grammar);
    grammar
}

impl NonDeterministicLR1Automaton {
    fn to_graphviz(&self) -> String {
        let mut result = String::from("digraph G {\nrankdir=\"LR\"\n");
        let mut ids: HashMap<&LR1Item, i32> = HashMap::new();
        for (cur, item) in self.edges.keys().enumerate() {
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
            ids.insert(item, cur as i32);
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

impl DetermenisticLR1Automaton {
    fn to_graphviz(&self) -> String {
        let mut result = String::from("digraph G {\nrankdir=\"LR\"\n");
        let mut ids: HashMap<&BTreeSet<LR1Item>, i32> = HashMap::new();
        for (cur, items) in self.edges.keys().enumerate() {
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
            ids.insert(items, cur as i32);
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

impl ParseTables {
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
