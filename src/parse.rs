use std::io;

use atom::Atom;

pub fn read_stdin_into(input: &mut String) -> &str {
    match io::stdin().read_line(input) {
        Ok(_) => input,
        Err(error) => {
            println!("error: {}", error);
            input
        }
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let toks: Vec<String> = input.replace("(", "( ").replace(")", " )")
        .replace("'", "' ").split_whitespace().into_iter()
        .map(|s| s.trim().to_string()).collect();
    toks
}

fn make_atom(input: &str) -> Result<Atom, String> {
    match input.parse::<i32>() {
        Ok(atom) => Ok(Atom::Int(atom)),
        Err(_) => Ok(Atom::Symbol(input.to_string())),
    }
}

fn read_list(mut tokens: &mut Vec<String>) -> Result<Vec<Atom>, String> {
    let mut list: Vec<Atom> = Vec::new();
    let mut compare: String = tokens[0].clone();
    while compare != ")" {
        list.push(try!(read_from_tokens(tokens)));
        if tokens.len() == 0 {
            return Err("Missing right paren".to_string())
        }
        compare = tokens[0].clone();
    }
    tokens.remove(0); // Remove ')'
    Ok(list)
}

pub fn read_from_tokens(mut tokens: &mut Vec<String>) -> Result<Atom, String> {
    if tokens.len() == 0 {
        return Err("Empty program".to_string());
    }

    let token = tokens.remove(0);
    match token.as_ref() {
        "(" => {
            Ok(Atom::List(try!(read_list(tokens))))
        },
        "'" => {
            let t = tokens.remove(0);
            if t != "(" {
                return Err("Expected ( after quote".to_string())
            }
            let mut list = try!(read_list(tokens));
            list.insert(0, Atom::Symbol("list".to_string()));
            Ok(Atom::List(list))
        },
        ")" => Err("Unexpected right paren".to_string()),
        _ => make_atom(token.as_ref()),
    }
}
