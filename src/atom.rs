use std::fmt::{Debug, Display, Formatter};
use std;
use environment::SchemeFnWrap;

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
    pub fn as_int(&self) -> Option<i32> {
        if let Atom::Int(i) = *self {
            return Some(i)
        } else {
            return None
        }
    }
    pub fn as_int_result(&self) -> Result<i32, String> {
        self.as_int().ok_or("Not an int".to_string())
    }
    pub fn as_list(&'a self) -> Option<&Vec<Atom>> {
        if let Atom::List(ref l) = *self {
            return Some(l)
        } else {
            return None
        }
    }
    pub fn as_list_result(&'a self) -> Result<&Vec<Atom>, String> {
        self.as_list().ok_or("Not a list".to_string())
    }
    pub fn as_callable_result(&'a self) -> Result<&SchemeFnWrap, String> {
        if let &Atom::Callable(ref callable) = self {
            return Ok(callable)
        } else {
            return Err("Not a callable".to_string())
        }
    }
    pub fn as_symbol(&self) -> Option<&String> {
        if let &Atom::Symbol(ref sym) = self {
            return Some(sym)
        }
        return None
    }
    pub fn as_symbol_result(&self) -> Result<&String, String> {
        self.as_symbol().ok_or("Not a symbol".to_string())
    }
}

impl Debug for Atom {
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result {
        use self::Atom::*;
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
impl Display for Atom {
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result {
        use self::Atom::*;
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