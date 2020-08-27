use super::environment::*;
use super::scmObject::*;

pub fn eval(input: ScmObject, mut env: &mut ScmEnvironment) -> ScmObject {
    let a = input.clone();
    match input {
        ScmObject::CONS(cons) => {
            let func = *cons.car;
            let eval_func = eval(func, env);

            match eval_func {
                ScmObject::FN(function) => build_in_functions(function, *cons.cdr, &mut env),
                ScmObject::Syntax(syntax) => build_in_syntax(syntax, *cons.cdr, &mut env),
                ScmObject::USERFN(function) => user_function(function, *cons.cdr, &mut env),
                _ => ScmObject::ERROR(String::from("not a func")),
            }
        }
        ScmObject::SYMBOL(_symbole) => {
            let result = env.get(a);
            if let ScmObject::None = result {
                return ScmObject::ERROR(String::from("Symbole not found"));
            } else {
                return result;
            }
        }
        _ => input,
    }
}

fn build_in_functions(
    function: ScmBuildInFunction,
    args_list: ScmObject,
    env: &mut ScmEnvironment,
) -> ScmObject {
    let mut arg_count: i64 = 1;

    match function.tag {
        BuildInFunction::Plus => {
            if let ScmObject::CONS(cons) = args_list {
                let mut arg = *cons.car;
                let mut restlist = *cons.cdr;
                let mut sum: i64 = 0;

                while arg_count <= function.num_args {
                    if let ScmObject::NIL = restlist {
                        if arg_count != function.num_args {
                            return ScmObject::ERROR(String::from(
                                "Plus have not the right number of args",
                            ));
                        }
                    } else {
                        if arg_count == function.num_args {
                            return ScmObject::ERROR(String::from(
                                "Plus have not the right number of args",
                            ));
                        }
                    }

                    while let ScmObject::SYMBOL(_symbole) = arg.clone() {
                        arg = eval(arg, env);
                    }

                    if let ScmObject::NUMBER(number) = arg {
                        sum = sum + number;
                    } else {
                        return ScmObject::ERROR(String::from("arg is not a number"));
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = restlist {
                        arg = *cons.car;
                        restlist = *cons.cdr;
                    }
                }
                return ScmObject::NUMBER(sum);
            }
        }
        BuildInFunction::Minus => {
            if let ScmObject::CONS(cons) = args_list {
                let mut arg = *cons.car;
                let mut restlist = *cons.cdr;
                let mut result: i64 = 0;
                while arg_count <= function.num_args {
                    if let ScmObject::NIL = restlist {
                        if arg_count != function.num_args {
                            return ScmObject::ERROR(String::from(
                                "Minus have not the right number of args",
                            ));
                        }
                    } else {
                        if arg_count == function.num_args {
                            return ScmObject::ERROR(String::from(
                                "Minus have not the right number of args",
                            ));
                        }
                    }

                    while let ScmObject::SYMBOL(_symbole) = arg.clone() {
                        arg = eval(arg, env);
                    }

                    if let ScmObject::NUMBER(number) = arg {
                        if arg_count == 1 {
                            result = number;
                        } else {
                            result = result - number;
                        }
                    } else {
                        return ScmObject::ERROR(String::from("arg is not a number"));
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = restlist {
                        arg = *cons.car;
                        restlist = *cons.cdr;
                    }
                }
                return ScmObject::NUMBER(result);
            }
        }
        _ => {
            return ScmObject::ERROR(String::from("Func not implement"));
        }
    }
    return ScmObject::ERROR(String::from("error in build in func"));
}

fn build_in_syntax(
    syntax: ScmBuildInSyntax,
    args_list: ScmObject,
    env: &mut ScmEnvironment,
) -> ScmObject {
    match syntax.tag {
        BuildInSyntax::Quote => {
            if let ScmObject::CONS(cons) = args_list {
                if let ScmObject::NIL = *cons.cdr {
                    let a: ScmObject = *cons.car;
                    return a;
                } else {
                    return ScmObject::ERROR(String::from(
                        "Quote restlist has more than one element",
                    ));
                }
            } else {
                return ScmObject::ERROR(String::from("Quote restlist is no list"));
            }
        }
        BuildInSyntax::Define => {
            if let ScmObject::CONS(cons) = args_list {
                if let ScmObject::CONS(value) = *cons.cdr {
                    if let ScmObject::NIL = *value.cdr {
                        let sym = *cons.car;
                        let expr = eval(*value.car, env);

                        env.define(sym, expr);

                        return ScmObject::Void;
                    } else {
                        return ScmObject::ERROR(String::from(
                            "Quote restlist more then tow element",
                        ));
                    }
                } else {
                    return ScmObject::ERROR(String::from("Quote restlist only one element"));
                }
            } else {
                return ScmObject::ERROR(String::from("Quote restlist is no list"));
            }
        }
        BuildInSyntax::Set => {
            if let ScmObject::CONS(cons) = args_list {
                if let ScmObject::CONS(value) = *cons.cdr {
                    if let ScmObject::NIL = *value.cdr {
                        let sym = *cons.car;
                        let expr = *value.car;

                        env.set(sym, expr);

                        return ScmObject::Void;
                    } else {
                        return ScmObject::ERROR(String::from(
                            "Quote restlist more then tow element",
                        ));
                    }
                } else {
                    return ScmObject::ERROR(String::from("Quote restlist only one element"));
                }
            } else {
                return ScmObject::ERROR(String::from("Quote restlist is no list"));
            }
        }
        BuildInSyntax::Lambda => {
            let arg_list;
            let body_list;
            let cdr;

            if let ScmObject::CONS(cons) = args_list {
                arg_list = *cons.car;
                cdr = *cons.cdr;
            } else {
                return ScmObject::ERROR(String::from("need mind 2 arg (there are 0)"));
            }

            if let ScmObject::CONS(cons) = cdr {
                body_list = *cons.car;
            } else {
                return ScmObject::ERROR(String::from("need mind 2 arg (there are 1)"));
            }
            // TODO: env
            return ScmObject::new_user_fn(None, arg_list, body_list, env.clone());
        }
        _ => {
            return ScmObject::ERROR(String::from("not a Syntax"));
        }
    }
}

fn user_function(
    function: ScmUserFunction,
    arg_list: ScmObject,
    env: &mut ScmEnvironment,
) -> ScmObject {
    let mut env_new = ScmEnvironment::new();
    env_new.set_parrent_env(env);

    let mut next_argument: Cons;

    if let ScmObject::CONS(cons) = arg_list {
        next_argument = cons;
    } else {
        return ScmObject::ERROR(String::from("arg_list is not a List"));
    }

    //eval args
    // loop {
    //     let eval_arg = eval(*next_argument.car, env);

    //     match *next_argument.cdr {
    //         ScmObject::NIL => break,
    //         ScmObject::CONS(cons) => next_argument = cons,
    //         _ => return ScmObject::ERROR(String::from("Error Cons cdr is not Cons or Nil")),
    //     }
    // }

    let mut next_argument_list: Cons;

    if let ScmObject::CONS(cons) = *function.arg_list {
        next_argument_list = cons;
    } else {
        return ScmObject::ERROR(String::from("Function arg_list is not a List"));
    }

    // define args
    loop {
        env_new.define(*next_argument_list.car, *next_argument.car);

        match *next_argument_list.cdr {
            ScmObject::NIL => break,
            ScmObject::CONS(cons) => next_argument_list = cons,
            _ => return ScmObject::ERROR(String::from("Error Cons cdr is not Cons or Nil")),
        }

        match *next_argument.cdr {
            ScmObject::NIL => break,
            ScmObject::CONS(cons) => next_argument = cons,
            _ => return ScmObject::ERROR(String::from("Error Cons cdr is not Cons or Nil")),
        }
    }

    let mut next_body: Cons;

    // if let ScmObject::CONS(cons) = *function.body_list {
    //     next_body = cons;
    // } else {
    //     return ScmObject::ERROR(String::from("Function arg_list is not a List"));
    // }

    //eval
    //loop {
    return eval(*function.body_list, &mut env_new);

    // match *next_body.cdr {
    //     ScmObject::NIL => return re,
    //     ScmObject::CONS(cons) => next_body = cons,
    //     _ => return ScmObject::ERROR(String::from("Error Cons cdr is not Cons or Nil")),
    // }
    //}
}
