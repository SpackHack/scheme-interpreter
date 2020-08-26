use super::scmObject::*;

pub struct ScmEnvironment {
    pub parent_env: Box<Option<ScmEnvironment>>,
    pub bindings: Vec<ScmObject>,
}

impl ScmEnvironment {
    pub fn new() -> Self {
        ScmEnvironment {
            bindings: Vec::new(),
            parent_env: Box::from(None),
        }
    }

    pub fn define(&mut self, key: ScmObject, value: ScmObject) {
        for elem in &self.bindings {
            if let ScmObject::CONS(mut cons) = &elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value;
                    return;
                }
            }
        }
        &self.bindings.push(ScmObject::new_cons(key, value));
    }

    pub fn set(&self, key: ScmObject, value: ScmObject) {
        for ref elem in &self.bindings {
            if let ScmObject::CONS(mut cons) = elem {
                if (*cons.car).equal(&key) {
                    *cons.cdr = value;
                    return;
                }
            }
        }
        match *self.parent_env {
            Some(ref env) => env.set(key, value),
            None => {}
        }
    }

    pub fn get(&self, key: ScmObject) -> ScmObject {
        for elem in &self.bindings {
            if let ScmObject::CONS(cons) = &elem {
                if (*cons.car).equal(&key) {
                    return *cons.cdr.clone();
                }
            }
        }

        ScmObject::ERROR(String::from("Variable not found"))
    }

    pub fn print() {}
}

// pub fn define_environment(environment: &mut ScmEnvironment, key: ScmObject, val: ScmObject) {
//     // for elem in environment.bindings.iter() {
//     //     if let ScmObject::CONS(mut cons) = elem {
//     //         if scm_equal(&key, &*cons.car) {
//     //             *cons.cdr = val;
//     //             return;
//     //         }
//     //     }
//     // }
//     environment.bindings.push(ScmObject::new_cons(key, val));
//     environment.num_bindings = environment.num_bindings + 1;
// }

// pub fn set_environment(env: &mut ScmEnvironment, key: ScmObject, val: ScmObject) -> ScmObject {
//     ScmObject::ERROR(String::from("set env not imple"))
// }

// pub fn get_environment(env: &ScmEnvironment, key: ScmObject) -> ScmObject {
//     for elem in env.bindings.iter() {
//         if let ScmObject::CONS(cons) = &elem {
//             if scm_equal(&key, &*cons.car) {
//                 return *cons.cdr.clone();
//             }
//         }
//     }
//     ScmObject::None
// }

// pub fn print_environment() {}
