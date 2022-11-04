#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_calculator_lexer() {
        let mut lexer = Lexer::new("(1+2)*3-4/5");
        let tokens = lexer.get_tokens().unwrap();
        println!("{:?}", tokens);
    }

    #[test]
    fn test_calculator_parser() {
        let mut lexer = Lexer::new("(1+2)*3-4/5");
        let tokens = lexer.get_tokens().unwrap();
        let tables = parser::get_parse_tables();
        let tree = ParseTree::from_tables_and_tokens(&tables, &tokens).unwrap();
        println!("{}", tree.to_graphviz());
    }
}