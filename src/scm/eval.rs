use scheme_lib::*;

pub fn eval(input: ScmObject, env: ScmObject) -> (ScmObject, ScmObject) {
    let a = input.clone();
    match input {
        ScmObject::CONS(cons) => {
            let func = *cons.car;
            let (eval_func, env) = eval(func, env);

            match eval_func {
                ScmObject::FN(function) => build_in_functions(function, *cons.cdr, env),
                ScmObject::Syntax(syntax) => build_in_syntax(syntax, *cons.cdr, env),
                ScmObject::USERFN(function) => {
                    (ScmObject::new_error(String::from("User FN ")), env)
                }
                _ => (ScmObject::new_error(String::from("not a func")), env),
            }
        }
        ScmObject::SYMBOL(symbole) => super::environment::getEnvironment(env, a),
        _ => (input, env),
    }
}

fn build_in_functions(
    function: ScmBuildInFunction,
    argslist: ScmObject,
    env: ScmObject,
) -> (ScmObject, ScmObject) {
    let mut arg_count: i64 = 1;

    match function.tag {
        BuildInFunction::Plus => {
            if let ScmObject::CONS(cons) = argslist {
                let mut arg = *cons.car;
                let mut restlist = *cons.cdr;
                let mut sum: i64 = 0;

                while arg_count <= function.numArgs {
                    if let ScmObject::NIL = restlist {
                        if arg_count != function.numArgs {
                            return (
                                ScmObject::new_error(String::from(
                                    "Plus have not the right number of args",
                                )),
                                env,
                            );
                        }
                    } else {
                        if arg_count == function.numArgs {
                            return (
                                ScmObject::new_error(String::from(
                                    "Plus have not the right number of args",
                                )),
                                env,
                            );
                        }
                    }

                    if let ScmObject::NUMBER(number) = arg {
                        sum = sum + number;
                    } else {
                        return (
                            ScmObject::new_error(String::from("arg is not a number")),
                            env,
                        );
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = restlist {
                        arg = *cons.car;
                        restlist = *cons.cdr;
                    }
                }
                return (ScmObject::new_number(sum), env);
            }
        }
        BuildInFunction::Minus => {
            if let ScmObject::CONS(cons) = argslist {
                let mut arg = cons.car;
                let mut restlist = cons.cdr;
                let mut result: i64 = 0;
                while arg_count <= function.numArgs {
                    if let ScmObject::NIL = *restlist {
                        if arg_count != function.numArgs {
                            return (
                                ScmObject::new_error(String::from(
                                    "Minus have not the right number of args",
                                )),
                                env,
                            );
                        }
                    } else {
                        if arg_count == function.numArgs {
                            return (
                                ScmObject::new_error(String::from(
                                    "Minus have not the right number of args",
                                )),
                                env,
                            );
                        }
                    }

                    if let ScmObject::NUMBER(number) = *arg {
                        if arg_count == 1 {
                            result = number;
                        } else {
                            result = result - number;
                        }
                    } else {
                        return (
                            ScmObject::new_error(String::from("arg is not a number")),
                            env,
                        );
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = *restlist {
                        arg = cons.car;
                        restlist = cons.cdr;
                    }
                }
                return (ScmObject::new_number(result), env);
            }
        }
        _ => {
            return (
                ScmObject::new_error(String::from("Func not implement")),
                env,
            );
        }
    }
    return (
        ScmObject::new_error(String::from("error in build in func")),
        env,
    );
}

fn build_in_syntax(
    syntax: ScmBuildInSyntax,
    argslist: ScmObject,
    mut env: ScmObject,
) -> (ScmObject, ScmObject) {
    match syntax.tag {
        BuildInSyntax::Quote => {
            if let ScmObject::CONS(cons) = argslist {
                if let ScmObject::NIL = *cons.cdr {
                    let a: ScmObject = *cons.car;
                    return (a, env);
                } else {
                    return (
                        ScmObject::new_error(String::from(
                            "Quote restlist has more than one element",
                        )),
                        env,
                    );
                }
            } else {
                return (
                    ScmObject::new_error(String::from("Quote restlist is no list")),
                    env,
                );
            }
        }
        BuildInSyntax::Define => {
            if let ScmObject::CONS(cons) = argslist {
                if let ScmObject::CONS(value) = *cons.cdr {
                    if let ScmObject::NIL = *value.cdr {
                        let sym = *cons.car;
                        let expr = *value.car;

                        env = super::environment::define_enviroment(env, sym, expr);

                        return (ScmObject::Void, env);
                    } else {
                        return (
                            ScmObject::new_error(String::from(
                                "Quote restlist more then tow element",
                            )),
                            env,
                        );
                    }
                } else {
                    return (
                        ScmObject::new_error(String::from("Quote restlist only one element")),
                        env,
                    );
                }
            } else {
                return (
                    ScmObject::new_error(String::from("Quote restlist is no list")),
                    env,
                );
            }
        }
        _ => {
            return (ScmObject::new_error(String::from("not a Syntax")), env);
        }
    }
}
