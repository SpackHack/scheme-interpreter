use super::scm_object::*;

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
                    return;
                }
            }
        }
        &self.bindings.push(ScmObject::new_cons(key, value.clone()));
    }

    pub fn set(&mut self, key: ScmObject, value: ScmObject) {
        for elem in self.bindings.iter_mut() {
            if let ScmObject::Cons(cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value.clone();
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
        ScmObject::Error(String::from("Variable not found"))
    }

    pub fn print() {}
}
