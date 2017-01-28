extern crate rust_scheme;
use rust_scheme::interpreter::run_program;

pub fn test_program(program: &str, expected: &str) {
    match run_program(program) {
        Ok(result) => assert_eq!(result, expected.to_string()),
        Err(_) => panic!()
    }
}

pub fn test_error(program: &str) {
    match run_program(program) {
        Ok(_) => panic!(),
        Err(_) => assert!(true)
    }
}

pub fn test_error_msg(program: &str, expected: &str) {
    match run_program(program) {
        Ok(_) => panic!(),
        Err(msg) => assert_eq!(msg, expected.to_string())
    }
}

#[test]
fn test_add() {
    test_program("(+ 1 2)", "3")
}

#[test]
fn test_multiply() {
    test_program("(* 3 4)", "12")
}

#[test]
fn test_subtract() {
    test_program("(- 3 10)", "-7")
}

#[test]
fn test_divide() {
    test_program("(/ 20 4)", "5")
}

#[test]
fn test_gt() {
    test_program("(> 10 5)", &format!("{}", true));
    test_program("(> 5 10)", &format!("{}", false));
    test_program("(> 5 5)", &format!("{}", false))
}

#[test]
fn test_lt() {
    test_program("(< 10 5)", &format!("{}", false));
    test_program("(< 5 10)", &format!("{}", true));
    test_program("(< 5 5)", &format!("{}", false))
}

#[test]
fn test_ge() {
    test_program("(>= 10 5)", &format!("{}", true));
    test_program("(>= 5 10)", &format!("{}", false));
    test_program("(>= 5 5)", &format!("{}", true))
}

#[test]
fn test_le() {
    test_program("(<= 10 5)", &format!("{}", false));
    test_program("(<= 5 10)", &format!("{}", true));
    test_program("(<= 5 5)", &format!("{}", true))
}

#[test]
fn test_eq() {
    test_program("(= 10 10)", &format!("{}", true));
    test_program("(= 15 10)", &format!("{}", false));
    test_program("(= 10 15)", &format!("{}", false))
}

#[test]
fn test_abs() {
    test_program("(abs -10)", "10");
    test_program("(abs 10)", "10")
}

#[test]
fn test_list_and_append() {
    test_program("(append (list 1 2 3) (list 4 5 6))", "(1 2 3 4 5 6)");
    test_program("(append (list) (list))", "()");
    test_program("(append (list 1 2 3) (list))", "(1 2 3)");
    test_program("(append (list) (list 1 2 3))", "(1 2 3)");
    test_program("(append (list 1 2 3) (list 4 5 6) (list 7) (list 8 9))", "(1 2 3 4 5 6 7 8 9)")
}

#[test]
fn test_quote() {
    test_program("'(1 2 3)", "(1 2 3)");
    test_program("(list? '(1 2 3))", "true");
    test_program("(append (list 1 2 3) '(4 5 6))", "(1 2 3 4 5 6)")
}

#[test]
fn test_apply() {
    test_program("(apply + (list 1 2 3))", "6");
    test_program("(apply + 1 2 (list 3 4))", "10")
}

#[test]
fn test_begin() {
    test_program("(begin 1 2 3)", "3");
    test_program("(begin 1 2 (begin 1 2 (+ 1 2)))", "3");
}

#[test]
fn test_car() {
    test_program("(car (list 1 2))", "1");
    test_program("(car (list (list 1 2) 3))", "(1 2)");
    test_error("(car 1)");
}

#[test]
fn test_cdr() {
    test_program("(cdr (list 1 2))", "(2)");
    test_program("(cdr (list 1 (list 1 2)))", "((1 2))");
    test_error("(cdr 1)")
}

#[test]
fn test_is_eq() {
    test_program("(eq? 1 1)", "true");
    test_program("(eq? (list 1 2) (list 1 2))", "false");
    test_program("(eq? (list 1 2) (list 1 1))", "false");
}

#[test]
fn test_is_equal() {
    test_program("(equal? 1 1)", "true");
    test_program("(equal? (list 1 2) (list 1 2))", "true");
    test_program("(equal? (list 1 2) (list 1 1))", "false");
    test_program("(equal? 1 2)", "false");
}

#[test]
fn test_is_list() {
    test_program("(list? (list 1 2))", "true");
    test_program("(list? (+ 1 2))", "false");
    test_program("(list? 1)", "false");
}

#[test]
fn test_define() {
    test_program("(begin (define x 5) (+ x 6))", "11");
    test_program("(begin (define (add2 y) (+ y 2)) (add2 3))", "5")
}

#[test]
fn test_recursion_simple() {
    test_program("(begin (define (fact x) (if (< x 2) x (* x (fact (- x 1))))) (fact 5))", "120")
}

#[test]
fn test_misc() {
    test_program("( list (+ 34 6) ( * 2 1 )  )", "(40 2)")
}

/* Failure tests */

#[test]
fn test_mismatch_paren() {
    test_error_msg("(begin (define (fact x) (* x (fact (- x 1))) (fact 5))", "Missing right paren")
}