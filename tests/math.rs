extern crate llisp;

mod tests {
    #[test]
    fn math_example_1() {
        use llisp::ast::ast::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(
            inter.interpret(&"(+ 1 2 3 (* 3 2 (- 4 2)))".to_string()),
            Cell::Number(18)
        )
    }

    #[test]
    fn math_example_2() {
        use llisp::ast::ast::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(
            inter.interpret(&"(* -2 3 (+ 3 -4))".to_string()),
            Cell::Number(6)
        )
    }
}
