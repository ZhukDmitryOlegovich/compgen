use generator::parser::ParseTables;
use std::io;
use std::process;

fn main() {
    let mut grammar = String::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        grammar.push_str(&line);
        grammar.push('\n');
    }
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
