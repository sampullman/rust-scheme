#![allow(unused_variables)]
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use Atom;
use interpreter::{evaluate, execute_fn};

#[derive(Clone, PartialEq)]
pub struct SchemeLambda {
    pub name: String,
    pub arg_list: Vec<String>,
    pub body: Vec<Atom>,
}

pub type SchemeFn = fn(env: Rc<RefCell<Environment>>, Vec<Atom>) -> Result<Atom, String>;

// Necessary because I could not find a way to implement clone for SchemeFn
// https://github.com/rust-lang/rust/issues/24000
pub enum SchemeFnWrap {
    Fn(SchemeFn),
    Lambda(SchemeLambda)
}

impl Clone for SchemeFnWrap {
    fn clone(&self) -> SchemeFnWrap {
        match self {
            &SchemeFnWrap::Fn(func) => SchemeFnWrap::Fn(func),
            &SchemeFnWrap::Lambda(ref lambda) => SchemeFnWrap::Lambda(lambda.clone())
        }
    }
}

impl PartialEq for SchemeFnWrap {
    fn eq(&self, other: &SchemeFnWrap) -> bool {
        unsafe {
            mem::transmute::<_, usize>(self) == mem::transmute::<_, usize>(other)
        }
    }
}

impl SchemeLambda {
    pub fn new(name: String, arg_list: Vec<String>, body: Vec<Atom>) -> SchemeLambda {
        SchemeLambda {name: name, arg_list: arg_list, body: body}
    }

    pub fn evaluate(self, env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
        if args.len() != self.arg_list.len() {
            return Err(format!("{} requires {} arguments", self.name, self.arg_list.len()))
        }
        let new_env = env_spawn_child(env);
        let mut i = 0;
        for name in self.arg_list.into_iter() {
            env_set(new_env.clone(), name, args[i].clone());
            i += 1;
        }
        let mut body_iter = self.body.iter();
        for statement in body_iter.by_ref().take(self.body.len()-1) {
            try!(evaluate(statement.clone(), new_env.clone()));
        }
        evaluate(body_iter.last().unwrap_or(&Atom::Nil).clone(), new_env)
    }
}

fn scheme_add(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    return Ok(Atom::Int(args.iter().filter_map(|a| a.as_int()).fold(0, |a, b| a + b)));
}

fn scheme_multiply(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    return Ok(Atom::Int(args.iter().filter_map(|a| a.as_int()).fold(1, |a, b| a * b)));
}

fn scheme_subtract(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to subtract {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Int(arg1 - arg2));
    }
}

fn scheme_divide(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to divide {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Int(arg1 / arg2));
    }
}

fn scheme_gt(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to > {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 > arg2));
    }
}

fn scheme_lt(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to < {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 < arg2));
    }
}

fn scheme_ge(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to >= {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 >= arg2));
    }
}

fn scheme_le(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to <= {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 <= arg2));
    }
}

fn scheme_eq(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to = {}", args.len()));
    } else {
        let arg1 = try!(args[0].as_int_result());
        let arg2 = try!(args[1].as_int_result());
        return Ok(Atom::Bool(arg1 == arg2));
    }
}

fn scheme_abs(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 1 {
        return Err(format!("Invalid number of operands to abs {}", args.len()));
    } else {
        let arg = try!(args[0].as_int_result());
        return Ok(Atom::Int(arg.abs()));
    }
}

fn scheme_append(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    let mut l1 = vec![];
    for arg in args {
        let mut lst = try!(arg.as_list_result()).clone();
        l1.append(&mut lst);
    }
    return Ok(Atom::List(l1))
}

fn scheme_apply(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() < 2 {
        return Err(format!("Invalid number of operands to apply {}", args.len()))
    }
    let func = try!(args[0].as_callable_result()).clone();
    let mut lst: Vec<Atom> = args.clone().into_iter().skip(1).take(args.len()-1).collect();
    lst.append(&mut try!(args[args.len()-1].as_list_result()).clone());
    execute_fn(func, lst, env)
}

fn scheme_begin(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
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

fn scheme_car(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
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

fn scheme_cdr(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
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

fn scheme_cons(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to cons {}", args.len()))
    } else {
        return Ok(Atom::Cons(Box::new(args[0].clone()), Box::new(args[1].clone())))
    }
}

// TODO -- Shore up implementation
fn scheme_is_eq(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
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

fn scheme_is_equal(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    if args.len() != 2 {
        return Err(format!("Invalid number of operands to equal? {}", args.len()))
    } else {
        return Ok(Atom::Bool(args[0] == args[1]))
    }
}

fn scheme_list(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    Ok(Atom::List(args))
}

fn scheme_is_list(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
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

fn scheme_let(env: Rc<RefCell<Environment>>, args: Vec<Atom>) -> Result<Atom, String> {
    Err("let not defined".to_string())
}

#[derive(PartialEq)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    definitions: HashMap<String, Atom>,
}

// Wrappers to avoid working directly with Rc/RefCell
pub fn env_get(env: &Rc<RefCell<Environment>>, s: &String) -> Result<Atom, String>  {
    env.as_ref().borrow().get_symbol(s)
}

pub fn env_set(env: Rc<RefCell<Environment>>, symbol: String, atom: Atom) {
    env.as_ref().borrow_mut().set_symbol(symbol, atom)
}

pub fn env_spawn_child(env: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
    Rc::new(RefCell::new(Environment { parent: Some(env), definitions: HashMap::new() }))
}

impl Environment {
    pub fn new() -> Environment {
        Environment { parent: None, definitions: HashMap::new() }
    }

    pub fn standard_env() -> Rc<RefCell<Environment>> {
        let mut env = Environment::new();
        env.set_symbol("+".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_add)));
        env.set_symbol("*".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_multiply)));
        env.set_symbol("-".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_subtract)));
        env.set_symbol("/".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_divide)));
        env.set_symbol(">".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_gt)));
        env.set_symbol("<".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_lt)));
        env.set_symbol(">=".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_ge)));
        env.set_symbol("<=".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_le)));
        env.set_symbol("=".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_eq)));
        env.set_symbol("abs".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_abs)));
        env.set_symbol("append".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_append)));
        env.set_symbol("apply".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_apply)));
        env.set_symbol("begin".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_begin)));
        env.set_symbol("car".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_car)));
        env.set_symbol("cdr".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_cdr)));
        env.set_symbol("cons".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_cons)));
        env.set_symbol("eq?".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_is_eq)));
        env.set_symbol("equal?".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_is_equal)));
        env.set_symbol("list".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_list)));
        env.set_symbol("list?".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_is_list)));
        env.set_symbol("let".to_string(), Atom::Callable(SchemeFnWrap::Fn(scheme_let)));
        Rc::new(RefCell::new(env))
    }

    pub fn set_symbol(&mut self, symbol: String, atom: Atom) {
        self.definitions.insert(symbol, atom);
    }

    pub fn get_symbol(&self, symbol: &String) -> Result<Atom, String> {
        if let Some(atom) = self.definitions.get(symbol) {
            Ok(atom.clone())
        } else {
            match self.parent {
                Some(ref parent) => env_get(&parent, symbol),
                None => Err(format!("Invalid definition {:?}", symbol))
            }
        }
    }
}
