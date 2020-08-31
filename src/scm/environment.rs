use super::printer::display_or_print;
use super::scm_object::*;

use std::io;
use std::io::Write;

// TODO: add hashing

#[derive(Clone)]
pub struct ScmEnvironment {
    pub parent_env: Option<Box<ScmEnvironment>>,
    pub bindings: Vec<ScmObject>,
}

impl ScmEnvironment {
    pub fn new() -> Self {
        ScmEnvironment {
            bindings: Vec::new(),
            parent_env: None,
        }
    }

    pub fn set_parent_env(&mut self, env: &ScmEnvironment) {
        self.parent_env = Some(Box::from(env.clone()));
    }

    pub fn define(&mut self, key: ScmObject, value: &ScmObject) {
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value.clone();
                    &self.update_user_function_env();
                    return;
                }
            }
        }
        &self.bindings.push(ScmObject::new_cons(key, value.clone()));
        &self.update_user_function_env();
    }

    pub fn set(&mut self, key: ScmObject, value: ScmObject) {
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value.clone();
                    &self.update_user_function_env();
                    break;
                }
            }
        }
        if let Some(e) = self.parent_env.iter_mut().next() {
            (*e).set(key, value)
        }
    }

    pub fn get(&mut self, key: ScmObject) -> ScmObject {
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    return *cons.cdr.clone();
                }
            }
        }
        if let Some(e) = self.parent_env.iter_mut().next() {
            return (*e).get(key);
        }
        println!("Symbole not found");
        self.print();
        ScmObject::Null
    }

    pub fn print(&mut self) {
        println!("Print Env:");
        for e in self.bindings.iter() {
            if let ScmObject::Cons(cons) = e {
                print!("Key: ");
                display_or_print(*cons.car.clone(), false);
                print!("\t\tValue:");
                display_or_print(*cons.cdr.clone(), false);
                println!();
            }
        }
        if let Some(mut s) = self.parent_env.clone() {
            println!("Parent Env");
            s.print();
            return;
        }
        io::stdout().flush().unwrap();
    }

    fn update_user_function_env(&mut self) {
        let e = self.clone();
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if let ScmObject::UserFunction(func) = *cons.cdr.clone() {
                    *cons.cdr =
                        ScmObject::new_user_fn(func.name, *func.arg_list, *func.body_list, e.clone());
                }
            }
        }
    }
}
