use generator::parser::ParseTables;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let grammar_file = args[1].clone();
    let grammar = fs::read_to_string(grammar_file).unwrap();
    let res = ParseTables::from_string(&grammar, generator::ParseTablesType::LALR);
    match res {
        Ok(tables) => {
            println!("{}", tables.to_rust_source());
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
