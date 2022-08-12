use serde::{Deserialize, Serialize};
use sexp;
use sexp::{Atom, Sexp};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Repl {
    definitions: Vec<Vec<String>>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: String);
}

#[wasm_bindgen]
impl Repl {
    pub fn eval(&mut self, input: String) -> String {
        let result = sexp::parse(&input);
        let exp = match result {
            Ok(rs) => rs,
            Err(_err) => return "error: could not parse S expression".to_string(),
        };
        match exp {
            Sexp::Atom(Atom::I(i)) => i.to_string(),
            Sexp::Atom(Atom::S(string)) => {
                for definition in self.definitions().iter() {
                    if definition[0] == string {
                        return definition[1].clone();
                    }
                }
                return string;
            }
            Sexp::List(v) => {
                let e1: Option<String>;
                let e2: Option<String>;
                if v.len() > 1 {
                    e1 = Some(Repl::eval(self, v[1].to_string()))
                } else {
                    e1 = None
                }
                if v.len() > 2 {
                    e2 = Some(Repl::eval(self, v[2].to_string()));
                } else {
                    e2 = None
                }
                match v.len() {
                    1 => return Repl::eval(self, v[0].to_string()),
                    3 => {
                        let arg1 = match e1 {
                            Some(rs) => rs,
                            None => return "error: invalid 1st argument".to_string(),
                        };
                        let arg2 = match e2 {
                            Some(rs) => rs,
                            None => return "error: invalid 2nd argument".to_string(),
                        };
                        match &v[0] {
                            Sexp::Atom(Atom::S(s)) if s == "define" => {
                                self.set_definitions(vec![arg1.to_string(), arg2.to_string()]);
                                return format!("defined {}", arg1);
                            }
                            Sexp::Atom(Atom::S(s)) if s == "+" => {
                                return (arg1.parse::<i64>().unwrap()
                                    + arg2.parse::<i64>().unwrap())
                                .to_string()
                            }
                            Sexp::Atom(Atom::S(s)) if s == "-" => {
                                return (arg1.parse::<i64>().unwrap()
                                    - arg2.parse::<i64>().unwrap())
                                .to_string()
                            }
                            Sexp::Atom(Atom::S(s)) if s == "*" => {
                                return (arg1.parse::<i64>().unwrap()
                                    * arg2.parse::<i64>().unwrap())
                                .to_string()
                            }
                            Sexp::Atom(Atom::S(s)) if s == "/" => {
                                return (arg1.parse::<i64>().unwrap()
                                    / arg2.parse::<i64>().unwrap())
                                .to_string()
                            }
                            _ => return format!("error: unknown operator {}", v[0]),
                        }
                    }
                    _ => return "error: invalid S expression".to_string(),
                };
            }
            _ => return "error: invalid S expression".to_string(),
        }
    }
    pub fn apply(&self, procedure: &str, arg1: &str, arg2: &str) {}

    #[wasm_bindgen(getter)]
    pub fn definitions(&self) -> Vec<Vec<String>> {
        self.definitions.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_definitions(&mut self, definition: Vec<String>) {
        self.definitions.push(definition)
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Repl {
        return Repl {
            definitions: vec![],
        };
    }
}
