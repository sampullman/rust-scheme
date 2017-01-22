
use std::cell::RefCell;
use std::rc::Rc;
use atom::Atom;
use environment::{Environment, SchemeFnWrap, SchemeLambda, env_get, env_set};
use parse::{tokenize, read_from_tokens};

pub fn run_program(program: &str) -> Result<String, String> {
    let env = Environment::standard_env();
    let mut tokens = tokenize(program);
    let ast = try!(read_from_tokens(&mut tokens));
    let result = try!(evaluate(ast, env));
    Ok(format!("{}", result))
}

pub fn evaluate<'a>(atom: Atom, env: Rc<RefCell<Environment>>) -> Result<Atom, String> {
    match atom {
        Atom::Nil => Ok(atom),
        Atom::Bool(_) => Ok(atom),
        Atom::Int(_) => Ok(atom),
        Atom::Callable(_) => Ok(atom),
        Atom::Symbol(s) => env_get(&env, &s),
        Atom::List(list) => {
            let list_clone = list.clone();
            let mut list_iter = list.into_iter();
            let first: Atom = try!(list_iter.next().ok_or("Ill-formed expression".to_string()));
            if let Some(result) = try!(check_special_forms(first.clone(), list_clone, env.clone())) {
                return Ok(result)
            }

            let callable = try!(evaluate(first.clone(), env.clone()));
            if let Atom::Callable(func_wrap) = callable {
                let mut args: Vec<Atom> = Vec::new();
                for arg in list_iter {
                    args.push(try!(evaluate(arg, env.clone())));
                }
                execute_fn(func_wrap, args, env)
            } else {
                Err(format!("Expected function, found {:?}", first))
            }
        },
        _ => Err(format!("Expected atom {:?}", atom))
    }
}

pub fn execute_fn(func_wrap: SchemeFnWrap, args: Vec<Atom>, env: Rc<RefCell<Environment>>) -> Result<Atom, String> {
    match func_wrap {
        SchemeFnWrap::Fn(func) => func(env, args),
        SchemeFnWrap::Lambda(lambda) => lambda.evaluate(env, args)
    }
}

fn eval_define(env: Rc<RefCell<Environment>>, arg_list: &Atom, body: Vec<Atom>) -> Result<Atom, String> {
    match arg_list.clone() {
        Atom::Symbol(sym) => {
            println!("Define var");
            if body.len() == 0 {
                env_set(env, sym, Atom::Nil)
            } else if body.len() == 1 {
                env_set(env, sym, body[0].clone())
            } else {
                return Err("Ill formed define!".to_string())
            }
            Ok(Atom::Nil)
        },
        Atom::List(args) => {
            println!("Define function");
            let mut args_iter = args.into_iter();
            let name_atom = try!(args_iter.next().ok_or("define needs a name".to_string()));
            let name = try!(name_atom.as_symbol().ok_or("define name must be a symbol".to_string()));
            let mut arg_names: Vec<String> = Vec::new();
            for arg_name_atom in args_iter {
                arg_names.push(try!(arg_name_atom.as_symbol().ok_or("Non-symbol in defing arg list")).clone());
            }
            let lambda = SchemeFnWrap::Lambda(SchemeLambda {name: name.clone(), arg_list: arg_names, body: body});
            env_set(env, name.clone(), Atom::Callable(lambda));
            Ok(Atom::Nil)
        }
        _ => Err("First argument to define must be a symbol or list".to_string())
    }
}

fn eval_if(env: Rc<RefCell<Environment>>, condition: &Atom, body: Vec<Atom>) -> Result<Atom, String> {
    let evaluated_condition = try!(evaluate(condition.clone(), env.clone()));
    if let Atom::Bool(cond) = evaluated_condition {
        if cond {
            evaluate(body[0].clone(), env.clone())
        } else {
            if body.len() == 2 {
                evaluate(body[1].clone(), env)
            } else {
                Err("Too many arguments to 'if'".to_string())
            }
        }
    } else {
        Err("First argument to 'if' must be a boolean".to_string())
    }
}

fn check_special_forms(atom_sym: Atom, args: Vec<Atom>, env: Rc<RefCell<Environment>>) -> Result<Option<Atom>, String> {
    if let Some(sym) = atom_sym.as_symbol() {
        let mut args_iter = args.into_iter();
        args_iter.next();
        match sym.as_ref() { // Handle special forms
            "define" => {
                let arg_list = try!(args_iter.next().ok_or("define takes 2 arguments".to_string()));
                let body: Vec<Atom> = args_iter.collect();
                return Ok(Some(try!(eval_define(env, &arg_list, body))))
            },
            "if" => {
                let condition = try!(args_iter.next().ok_or("if requires at least 2 arguments".to_string()));
                let body: Vec<Atom> = args_iter.collect();
                return Ok(Some(try!(eval_if(env, &condition, body))))
            }
            _ => return Ok(None)
        }
    }
    return Ok(None)
}
