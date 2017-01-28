#![allow(unused_variables)]
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use atom::Atom;
use interpreter::evaluate;
use builtins::*;

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
