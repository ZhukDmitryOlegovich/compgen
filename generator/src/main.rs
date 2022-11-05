use generator::parser::{self, ParseTables, ParseTree};
use generator::{self, DetermenisticLR1Automaton, Lexer, NonDeterministicLR1Automaton};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let grammar_file = args[1].clone();
    let grammar = fs::read_to_string(grammar_file).unwrap();
    let mut lexer = Lexer::new(&grammar);
    let tokens = lexer.get_tokens();
    let tables = parser::get_parse_tables();
    let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
    let encoded_grammar = generator::get_grammar_from_tree(&tree);
    let nfa = NonDeterministicLR1Automaton::from_grammar(&encoded_grammar);
    let dfa = DetermenisticLR1Automaton::from_non_deterministic(&nfa);
    let tables = ParseTables::from_automaton(&dfa, generator::ParseTablesType::LR1);
    println!("{}", tables.to_rust_source());
}
