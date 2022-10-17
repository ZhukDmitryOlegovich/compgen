use std::env;
use std::fs;

use compgen::DetermenisticLR1Automaton;
use compgen::Lexer;
use compgen::NonDeterministicLR1Automaton;
use compgen::get_grammar_from_tree;
use compgen::parser;
use compgen::parser::ParseTables;
use compgen::parser::ParseTree;

fn main() {
    let args: Vec<String> = env::args().collect();
    let grammar_file = args[1].clone();
    let grammar = fs::read_to_string(grammar_file).unwrap();
    let mut lexer = Lexer::new(&grammar);
    let tokens = lexer.get_tokens().unwrap();
    let tables = parser::get_parse_tables();
    let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
    let encoded_grammar = get_grammar_from_tree(&tree).unwrap();
    let nfa = NonDeterministicLR1Automaton::from_grammar(&encoded_grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    let tables = ParseTables::from_automaton(&dfa);
    println!("{}", tables.to_rust_source().unwrap());
}
