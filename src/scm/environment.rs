use scheme_lib::{*};

pub fn define_enviroment(env: ScmObject, key: ScmObject,  val: ScmObject) -> ScmObject{
    if let ScmObject::ENV(mut env) = env {
        env.bindings.push(ScmObject::new_cons(key, val));
        env.num_bindigs = env.num_bindigs + 1;
        return ScmObject::ENV(env);
    }
    env
}

pub fn setEnviroment() {}

pub fn getEnvironment(env: &ScmObject, key: ScmObject) -> ScmObject {
    
    if let ScmObject::ENV(e) = &env {
        for elem in &e.bindings {
            if let ScmObject::CONS(cons) = &elem {
                if scm_equal(&key, &*cons.car){
                    return *cons.cdr.clone();
                }
            }
        }
    }
    ScmObject::None
}

pub fn printEvironment() {}

