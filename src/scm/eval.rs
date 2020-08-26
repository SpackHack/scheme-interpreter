use super::environment::*;
use super::scmObject::*;

pub fn eval(input: ScmObject, mut env: &mut ScmEnvironment) -> ScmObject {
    let a = input.clone();
    match input {
        ScmObject::CONS(cons) => {
            let func = *cons.car;
            let eval_func = eval(func, env);

            match eval_func {
                ScmObject::FN(function) => build_in_functions(function, *cons.cdr, env),
                ScmObject::Syntax(syntax) => build_in_syntax(syntax, *cons.cdr, &mut env),
                ScmObject::USERFN(function) => ScmObject::ERROR(String::from("User FN ")),
                _ => ScmObject::ERROR(String::from("not a func")),
            }
        }
        ScmObject::SYMBOL(symbole) => {
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
    env: &ScmEnvironment,
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
                let mut arg = cons.car;
                let mut restlist = cons.cdr;
                let mut result: i64 = 0;
                while arg_count <= function.num_args {
                    if let ScmObject::NIL = *restlist {
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

                    if let ScmObject::NUMBER(number) = *arg {
                        if arg_count == 1 {
                            result = number;
                        } else {
                            result = result - number;
                        }
                    } else {
                        return ScmObject::ERROR(String::from("arg is not a number"));
                    }

                    arg_count = arg_count + 1;
                    if let ScmObject::CONS(cons) = *restlist {
                        arg = cons.car;
                        restlist = cons.cdr;
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
    mut env: &mut ScmEnvironment,
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
                        let expr = *value.car;

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
        _ => {
            return ScmObject::ERROR(String::from("not a Syntax"));
        }
    }
}
