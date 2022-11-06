use generator::parser::ParseTables;
use std::env;
use std::io;
use std::process;

struct Flags {
    help: bool,
    clr: bool,
}

fn main() {
    let mut flags = Flags {
        help: false,
        clr: false,
    };
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        if arg == "--clr" {
            flags.clr = true;
        } else if arg == "--help" {
            flags.help = true;
        } else {
            eprintln!("Unknown flag: {}", arg);
            process::exit(1);
        }
    }
    if flags.help {
        println!(
            "Usage: 
  compgen [options]

OPTIONS:
  --clr  generate CLR tables instead of LALR
  --help show list of command-line options"
        );
        return;
    }
    let tables_type = if flags.clr {
        generator::ParseTablesType::LR1
    } else {
        generator::ParseTablesType::LALR
    };
    let mut grammar = String::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        grammar.push_str(&line);
        grammar.push('\n');
    }
    let res = ParseTables::from_string(&grammar, tables_type);
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
