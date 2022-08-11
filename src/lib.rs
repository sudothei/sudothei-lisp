use sexp;
use sexp::{Atom, Sexp};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Repl {}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
impl Repl {
    pub fn eval(&self, input: &str) -> String {
        let s = sexp::parse(input).unwrap();
        let result: i64 = match s {
            Sexp::Atom(Atom::I(i)) => i,
            Sexp::List(v) => {
                let e1: &str = &Repl::eval(&self, &v[1].to_string());
                let e2: &str = &Repl::eval(&self, &v[2].to_string());
                let op: &str = match &v[0] {
                    Sexp::Atom(Atom::S(s)) if s == "+" => "plus",
                    Sexp::Atom(Atom::S(s)) if s == "-" => "minus",
                    Sexp::Atom(Atom::S(s)) if s == "*" => "mult",
                    Sexp::Atom(Atom::S(s)) if s == "/" => "div",
                    _ => return "error".to_string(),
                };
                match op {
                    "plus" => e1.parse::<i64>().unwrap() + e2.parse::<i64>().unwrap(),
                    "minus" => e1.parse::<i64>().unwrap() - e2.parse::<i64>().unwrap(),
                    "mult" => e1.parse::<i64>().unwrap() * e2.parse::<i64>().unwrap(),
                    "div" => e1.parse::<i64>().unwrap() / e2.parse::<i64>().unwrap(),
                    _ => return "error".to_string(),
                }
            }
            _ => return "error".to_string(),
        };
        result.to_string()
    }
    pub fn new() -> Repl {
        return Repl {};
    }
}
