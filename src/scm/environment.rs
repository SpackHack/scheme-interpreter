use super::printer::display_or_print;
use super::scm_object::*;

use std::io;
use std::io::Write;
use std::rc::Rc;

// TODO: add hashing

#[derive(Clone)]
pub struct ScmEnvironment {
    pub parent_env: Option<Rc<ScmEnvironment>>,
    pub bindings: Vec<ScmObject>,
}

impl ScmEnvironment {
    pub fn set_parent_env(&mut self, env: Rc<ScmEnvironment>) {
        self.parent_env = Some(Rc::clone(&env));
    }

    pub fn define(&mut self, key: ScmObject, value: &ScmObject) {
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value.clone();
                    return;
                }
            }
        }
        &self.bindings.push(ScmObject::new_cons(key, value.clone()));
    }

    pub fn set(&mut self, key: ScmObject, value: &ScmObject) -> ScmObject{
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value.clone();
                    return ScmObject::Void;
                }
            }
        }

        if let Some(mut e) = self.parent_env.iter_mut().next() {
            unsafe {
                return Rc::get_mut_unchecked(&mut e).set(key, value);
            }
        } else {
            return ScmObject::Error(String::from("Symbole not found"));
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
        if let Some(mut e) = self.parent_env.iter_mut().next() {
            unsafe {
                return Rc::get_mut_unchecked(&mut e).get(key);
            }
        }
        ScmObject::Error(String::from("Symbole not found"))
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
        if let Some(mut s) = self.parent_env.iter_mut().next() {
            println!("Parent Env");
            unsafe {
                Rc::get_mut_unchecked(&mut s).print();
            }
        }
        io::stdout().flush().unwrap();
    }
}
