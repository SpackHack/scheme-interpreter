use scheme_lib::*;
use super::environment::{*};

pub fn eval(input: ScmObject, env: &ScmObject) -> ScmObject {
    let scm = input.clone();

    match input {
        ScmObject::CONS(cons) => {
            let func = *cons.car;
            let eval_func = eval(func, env);

            match eval_func {
                ScmObject::FN(function) => {
                    return build_in_functions(function, *cons.cdr);
                }
                ScmObject::USERFN(function) => {}
                _ => return ScmObject::new_error(String::from("not a func")),
            }
        }
        ScmObject::SYMBOL(symbole) => {
            return super::environment::getEnvironment(env, scm);
        }
        _ => {
            return scm;
        }
    }
    return ScmObject::new_error(String::from("no eval"));
}

fn build_in_functions(function: ScmBuildInFunction, argslist: ScmObject) -> ScmObject {
    let mut arg_count: i64 = 1;

    match function.tag {
        BuildInFunction::QUOTE => {
            if let ScmObject::CONS(cons) = argslist {

                if let ScmObject::NIL = *cons.cdr {
                    let a: ScmObject = *cons.car; 
                    return a;
                } else {
                    return ScmObject::new_error(String::from(
                        "Quote restlist has more than one element",
                    ));
                }
            } else {
                return ScmObject::new_error(String::from("Quote restlist is no list"));
            }
        }
        BuildInFunction::FNPLUS => {
            if let ScmObject::CONS(cons) = argslist {
                let mut arg = *cons.car;
                let mut restlist = *cons.cdr;
                let mut sum: i64 = 0;

                while arg_count <= function.numArgs {
                    if let ScmObject::NIL = restlist {
                        if arg_count != function.numArgs {
                            return ScmObject::new_error(String::from(
                                "Plus have not the right number of args",
                            ));
                        }
                    } else {
                        if arg_count == function.numArgs {
                            return ScmObject::new_error(String::from(
                                "Plus have not the right number of args",
                            ));
                        }
                    }

                    if let ScmObject::NUMBER(number) = arg {
                        sum = sum + number;
                    } else {
                        return ScmObject::new_error(String::from("arg is not a number"));
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = restlist {
                        arg = *cons.car;
                        restlist = *cons.cdr;
                    }
                }
                return ScmObject::new_number(sum);
            }
        }
        BuildInFunction::FNMINUS => {
            if let ScmObject::CONS(cons) = argslist {
                let mut arg = cons.car;
                let mut restlist = cons.cdr;
                let mut result: i64 = 0;
                while arg_count <= function.numArgs {
                    if let ScmObject::NIL = *restlist {
                        if arg_count != function.numArgs {
                            return ScmObject::new_error(String::from(
                                "Minus have not the right number of args",
                            ));
                        }
                    } else {
                        if arg_count == function.numArgs {
                            return ScmObject::new_error(String::from(
                                "Minus have not the right number of args",
                            ));
                        }
                    }

                    if let ScmObject::NUMBER(number) = *arg {
                        if arg_count == 1 {
                            result = number;
                        } else {
                            result = result - number;
                        }
                    } else {
                        return ScmObject::new_error(String::from("arg is not a number"));
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = *restlist {
                        arg = cons.car;
                        restlist = cons.cdr;
                    }
                }
                return ScmObject::new_number(result);
            }
        }
        _ => {
            return ScmObject::new_error(String::from("Func not implement"));
        }
    }
    return ScmObject::new_error(String::from("error in build in func"));
}
