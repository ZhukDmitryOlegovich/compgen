use std::io::stdin;

use calculator::evaluate_from_string;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read line");
    let res = evaluate_from_string(&buf);
    match res {
        Ok(n) => {
            println!("{}", n);
        }
        Err(_) => {
            println!("Failed to evaluate expression");
        }
    }
}
