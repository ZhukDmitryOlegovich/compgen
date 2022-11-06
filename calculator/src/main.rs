use std::{io, process};

use calculator::evaluate_from_string;

fn main() {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        eprintln!("Failed to read input");
        process::exit(1);
    }
    let res = evaluate_from_string(&buf);
    match res {
        Ok(n) => {
            println!("{}", n);
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
