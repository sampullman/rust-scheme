extern crate rust_scheme;

use rust_scheme::interpreter::run_program;

fn main() {
    //let program = "(begin (define pi 3.14159) (define r 10) (* pi (* r r)))";
    //let program = "(begin (define (x y) (* y 5) (* y 10)) (x 8))";
    //let program = "(/ (- (+ 1 (+ 1 1)) 5) 2)";
    let program = "(begin (define (fact x) (* x (fact (- x 1))) (fact 5))";
    //let program = "(begin (define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))) (fib 20))";
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

