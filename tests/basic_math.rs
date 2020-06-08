extern crate llisp;

mod tests {

    #[test]
    fn simple_addition() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(+ 1 2 3)".to_string()), Cell::Number(6));
    }

    #[test]
    fn simple_wrong_addition() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_ne!(inter.interpret(&"(+ 1 2 3)".to_string()), Cell::Number(4));
    }

    #[test]
    fn simple_subtraction() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(- 3 2 3)".to_string()), Cell::Number(-2))
    }

    #[test]
    fn simple_mul() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(* 2 3 2)".to_string()), Cell::Number(12))
    }

    #[test]
    fn simple_div() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(/ 48 8 2)".to_string()), Cell::Number(3))
    }
}
