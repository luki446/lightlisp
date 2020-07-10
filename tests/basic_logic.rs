extern crate llisp;

mod tests {
    #[test]
    fn simple_if_true() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(if true 1 2)".to_string()), Cell::Number(1));
    }

    #[test]
    fn simple_if_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(if false 1 2)".to_string()), Cell::Number(2));
    }

    #[test]
    fn simple_if_false_empty() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(if false 1)".to_string()), Cell::Nil);
    }
}