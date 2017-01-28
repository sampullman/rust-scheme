#![allow(unused_variables)]
use std::cell::RefCell;
use std::rc::Rc;
use atom::Atom;
use environment::Environment;
use interpreter::{evaluate, execute_fn};

pub fn scheme_add(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    return Ok(Atom::Int(args.iter().filter_map(|a| a.as_int()).fold(0, |a, b| a + b)));
}

pub fn scheme_multiply(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    return Ok(Atom::Int(args.iter().filter_map(|a| a.as_int()).fold(1, |a, b| a * b)));
}

pub fn scheme_subtract(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to subtract {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Int(arg1 - arg2));
    }
}

pub fn scheme_divide(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to divide {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Int(arg1 / arg2));
    }
}

pub fn scheme_gt(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to > {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 > arg2));
    }
}

pub fn scheme_lt(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to < {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 < arg2));
    }
}

pub fn scheme_ge(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to >= {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 >= arg2));
    }
}

pub fn scheme_le(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to <= {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 <= arg2));
    }
}

pub fn scheme_eq(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to = {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 == arg2));
    }
}

pub fn scheme_abs(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 1 {
        return Err(format!("Invalid number of operands to abs {}", args.len()));
    } else {
        let arg = try!(args[0].as_int_result());
        return Ok(Atom::Int(arg.abs()));
    }
}

pub fn scheme_append(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    let mut l1 = vec![];
    for arg in args {
        let mut lst = try!(arg.as_list_result()).clone();
        l1.append(&mut lst);
    }
    return Ok(Atom::List(l1))
}

pub fn scheme_apply(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() < 2 {
        return Err(format!("Invalid number of operands to apply {}", args.len()))
    }
    let func = try!(args[0].as_callable_result()).clone();
    let mut lst: Vec<Atom> = args.clone().into_iter().skip(1).take(args.len()-1).collect();
    lst.append(&mut try!(args[args.len()-1].as_list_result()).clone());
    execute_fn(func, lst, env)
}

pub fn scheme_begin(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() == 0 {
        return Err("Begin requires at least one argument".to_string())
    }
    let args_len = args.len();
    let mut iter = args.into_iter();
    for arg in iter.by_ref().take(args_len-1) {
        try!(evaluate(arg.clone(), env.clone()));
    }
    return evaluate(iter.last().unwrap_or(Atom::Nil), env)
}

pub fn scheme_car(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 1 {
        return Err("CAR expects 1 argument".to_string())
    }
    match args[0].clone() {
        Atom::Cons(car, cdr) => return Ok(*car),
        Atom::List(atoms) => {
            if atoms.len() > 0 {
                return Ok(atoms[0].clone())
            } else {
                return Err("CAR expects a pair".to_string())
            }
        },
        _ => return Err("CAR expects a pair".to_string())
    }
}

pub fn scheme_cdr(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 1 {
        return Err("CDR expects 1 argument".to_string())
    }
    match args[0].clone() {
        Atom::Cons(car, cdr) => return Ok(*cdr),
        Atom::List(atoms) => {
            if atoms.len() > 0 {
                return Ok(Atom::List(atoms.into_iter().skip(1).collect()))
            } else {
                return Err("CDR expects a pair".to_string())
            }
        },
        _ => return Err("CDR expects a pair".to_string())
    }
}

pub fn scheme_cons(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to cons {}", args.len()))
    } else {
        return Ok(Atom::Cons(Box::new(args[0].clone()), Box::new(args[1].clone())))
    }
}

// TODO -- Shore up implementation
pub fn scheme_is_eq(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to eq? {}", args.len()))
    } else {
        return Ok(Atom::Bool(match &args[0] {
            &Atom::Bool(_) => args[0] == args[1],
            &Atom::Int(_) => args[0] == args[1],
            _ => &args[0] as *const Atom == &args[1] as *const Atom
        }))
    }
}

pub fn scheme_is_equal(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to equal? {}", args.len()))
    } else {
        return Ok(Atom::Bool(args[0] == args[1]))
    }
}

pub fn scheme_list(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    Ok(Atom::List(args))
}

pub fn scheme_is_list(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 1 {
        return Err(format!("Invalid number of operands to equal? {}", args.len()))
    } else {
        if let Some(_) = args[0].as_list() {
            return Ok(Atom::Bool(true))
        } else {
            return Ok(Atom::Bool(false))
        }
    }
}

pub fn scheme_let(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    Err("let not defined".to_string())
}