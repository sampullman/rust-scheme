#![allow(dead_code)]
use std::fmt::{Debug, Display, Formatter};

mod environment;
mod interpreter;
mod parse;
use environment::{Environment, SchemeFnWrap};
use parse::{tokenize, read_from_tokens};
use interpreter::evaluate;

#[derive(Clone, PartialEq)]
pub enum Atom {
    Bool(bool),
    Int(i32),
    Symbol(String),
    Cons(Box<Atom>, Box<Atom>),
    List(Vec<Atom>),
    Callable(SchemeFnWrap),
    Nil,
}

impl<'a> Atom {
    fn as_int(&self) -> Option<i32> {
        if let Atom::Int(i) = *self {
            return Some(i)
        } else {
            return None
        }
    }
    fn as_int_result(&self) -> Result<i32, String> {
        self.as_int().ok_or("Not an int".to_string())
    }
    fn as_list(&'a self) -> Option<&Vec<Atom>> {
        if let Atom::List(ref l) = *self {
            return Some(l)
        } else {
            return None
        }
    }
    fn as_list_result(&'a self) -> Result<&Vec<Atom>, String> {
        self.as_list().ok_or("Not a list".to_string())
    }
    fn as_callable_result(&'a self) -> Result<&SchemeFnWrap, String> {
        if let &Atom::Callable(ref callable) = self {
            return Ok(callable)
        } else {
            return Err("Not a callable".to_string())
        }
    }
    fn as_symbol(&self) -> Option<&String> {
        if let &Atom::Symbol(ref sym) = self {
            return Some(sym)
        }
        return None
    }
    fn as_symbol_result(&self) -> Result<&String, String> {
        self.as_symbol().ok_or("Not a symbol".to_string())
    }
}

impl<'a> Debug for Atom {
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result {
        use Atom::*;
        match self {
            &Bool(b) => write!(f, "'{}'", b),
            &Int(n) => write!(f, "'{}'", n),
            &Symbol(ref s) => write!(f, "'{}'", &**s),
            &Cons(ref car, ref cdr) => write!(f, "'({:?} . {:?})", &**car, &**cdr),
            &List(ref atoms) => write!(f, "{:?}", &**atoms),
            &Callable(_) => write!(f, "SchemeFn()"),
            &Nil => write!(f, "Nil"),
        }
    }
}

#[allow(unused_must_use)]
impl<'a> Display for Atom {
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result {
        use Atom::*;
        match self {
            &Bool(b) => write!(f, "{}", b),
            &Int(n) => write!(f, "{}", n),
            &Symbol(ref s) => write!(f, "{}", &**s),
            &Cons(ref car, ref cdr) => write!(f, "'({} . {})", &**car, &**cdr),
            &List(ref atoms) => {
                write!(f, "(");
                for (i, ref atom) in (&**atoms).iter().enumerate() {
                    if i == atoms.len()-1 {
                        write!(f, "{}", *atom);
                    } else {
                        write!(f, "{} ", *atom);
                    }
                }
                write!(f, ")")
            }
            &Callable(_) => write!(f, "SchemeFn()"),
            &Nil => write!(f, "Nil"),
        }
    }
}

fn main() {
    //let program = "(begin (define pi 3.14159) (define r 10) (* pi (* r r)))";
    //let program = "(begin (define (x y) (* y 5) (* y 10)) (x 8))";
    //let program = "(/ (- (+ 1 (+ 1 1)) 5) 2)";
    //let program = "(begin (define (fact x) (if (= x 0) 1 (* x (fact (- x 1))))) (fact 10))";
    let program = "(begin (define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))) (fib 30))";
    //let program = "(apply * 5 5 (list 4 5 6))";
    //let program = "( list (+ 34 6) ( * 2 1 )  )";
    //let mut input = String::new();
    //read_stdin_into(&mut input);
    //tokenize(program);
    match run_program(program) {
        Ok(result) => println!("Read program: {}", result),
        Err(err) => println!("Semantic error! {:?}", err),
    }
}

fn run_program(program: &str) -> Result<String, String> {
    let env = Environment::standard_env();
    let mut tokens = tokenize(program);
    let ast = try!(read_from_tokens(&mut tokens));
    let result = try!(evaluate(ast, env));
    Ok(format!("{}", result))
}

pub fn test_program(program: &str, expected: &str) {
    match run_program(program) {
        Ok(result) => assert_eq!(result, expected.to_string()),
        Err(_) => panic!()
    }
}

pub fn test_error(program: &str) {
    match run_program(program) {
        Ok(_) => panic!(),
        Err(_) => assert_eq!(true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        test_program("(begin (define (fact x) (* x (fact (- x 1))) (fact 5))", "5")
    }

    #[test]
    fn test_misc() {
        test_program("( list (+ 34 6) ( * 2 1 )  )", "(40 2)")
    }
}
