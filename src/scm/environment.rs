use super::printer::display_or_print;
use super::scm_object::*;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::rc::Rc;

#[derive(Clone)]
pub struct ScmEnvironment {
    pub parent_env: Option<Rc<ScmEnvironment>>,
    pub bindings: HashMap<String, ScmObject>,
}

impl ScmEnvironment {
    pub fn set_parent_env(&mut self, env: Rc<ScmEnvironment>) {
        self.parent_env = Some(Rc::clone(&env));
    }

    pub fn define(&mut self, key: ScmObject, value: &ScmObject) {
        if let ScmObject::Symbol(s) = key {
            self.bindings.insert(s, value.clone());
        }
    }

    pub fn set(&mut self, key: ScmObject, value: &ScmObject) -> ScmObject {
        if let ScmObject::Symbol(s) = key.clone() {
            if let Some(_) = self.bindings.get(&s) {
                self.bindings.insert(s, value.clone());
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
        if let ScmObject::Symbol(s) = key.clone() {
            if let Some(v) = self.bindings.get(&s) {
                return v.clone();
            }
        }

        if let Some(mut e) = self.parent_env.iter_mut().next() {
            unsafe {
                return Rc::get_mut_unchecked(&mut e).get(key);
            }
        }
        ScmObject::Null
    }

    pub fn print(&mut self) {
        println!("Print Env:");
        for (key, value) in self.bindings.iter() {
            print!("Key: {}", key);
            print!("\t\tValue:");
            display_or_print(value.clone(), false);
            println!();
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
