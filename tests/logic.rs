extern crate llisp;

mod tests {
    #[test]
    fn simple_equation() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(= 1 1)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_not_equal() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(<> 1 2)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_equation_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(= 1 2)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_not_equal_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(<> 1 1)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_lesser() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(< 1 2)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_lesser_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(< 1 1)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_greater() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(> 2 1)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_greater_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(> 1 2)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_lseq() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(<= 1 2)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_lseq_1() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(<= 1 1)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_lseq_2() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(<= 2 1)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_gteq_1() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(>= 2 1)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_gteq_2() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(>= 2 2)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_gteq_3() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(>= 1 2)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_and() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(&& true true)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_and_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(&& true false)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn simple_or() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(|| true false true)".to_string()), Cell::Bool(true));
    }

    #[test]
    fn simple_or_false() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(|| false false nil)".to_string()), Cell::Bool(false));
    }

    #[test]
    fn logic_1() {
        use llisp::ast::cells::Cell;
        use llisp::interpreter::Interpreter;
        let mut inter = Interpreter::new();

        assert_eq!(inter.interpret(&"(&& (= 123 123 123) (< 10 (* 10 10) (* 10 10 10)))".to_string()), Cell::Bool(false));
    }
}
