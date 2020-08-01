use scheme_lib::{*};

pub fn eval(input: ScmObject) -> ScmObject {
    let scm = input.clone();

    match input.value {
        ObjectType::CONS(cons) => {
            let func = *cons.car;
            let evalFunc = eval(func);

            match evalFunc.value {
                ObjectType::FN(function) => {
                    build_in_functions(function, *cons.cdr);
                } 
                ObjectType::USERFN(function) => {

                }
                _ => {
                    return ScmObject::new_error(String::from("not a func"))
                }
            }

            // match cons.car.value {
            //     ObjectType::SYMBOL(symbole) => match symbole.as_str() {
            //         "'" => {
            //             return build_in_functions(
            //                 ScmObject::new_fn(BuildInFunction::QUOTE, String::from("Quote"), 1),
            //                 *cons.cdr,
            //             )
            //         }
            //         "+" => {
            //             return build_in_functions(
            //                 ScmObject::new_fn(BuildInFunction::FNPLUS, String::from("Plus"), 2),
            //                 *cons.cdr,
            //             )
            //         }
            //         "minus" => {
            //             return build_in_functions(
            //                 ScmObject::new_fn(BuildInFunction::FNMINUS, String::from("Minus"), 2),
            //                 *cons.cdr,
            //             )
            //         }
            //         _ => {
            //             return scm;
            //         }
            //     },
            //     _ => {
            //         return scm;
            //     }
            // }
        }
        ObjectType::SYMBOL(symbole) => {}
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
                if let ObjectType::CONS(cons) = argslist.value {
                    if let ObjectType::NIL = cons.cdr.value {
                        return *cons.car;
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
                if let ObjectType::CONS(cons) = argslist.value {
                    let mut arg = cons.car;
                    let mut restlist = cons.cdr;
                    let mut sum: i64 = 0;

                    while arg_count <= function.numArgs {
                        if let ObjectType::NIL = restlist.value {
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

                        if let ObjectType::NUMBER(number) = arg.value {
                            sum = sum + number;
                        } else {
                            return ScmObject::new_error(String::from("arg is not a number"));
                        }

                        arg_count = arg_count + 1;
                        if let ObjectType::CONS(cons) = restlist.value {
                            arg = cons.car;
                            restlist = cons.cdr;
                        }
                    }
                    return ScmObject::new_number(sum);
                }
            }
            BuildInFunction::FNMINUS => {
                if let ObjectType::CONS(cons) = argslist.value {
                    let mut arg = cons.car;
                    let mut restlist = cons.cdr;
                    let mut result: i64 = 0;
                    while arg_count <= function.numArgs {
                        if let ObjectType::NIL = restlist.value {
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

                        if let ObjectType::NUMBER(number) = arg.value {
                            if arg_count == 1 {
                                result = number;
                            } else {
                                result = result - number;
                            }
                        } else {
                            return ScmObject::new_error(String::from("arg is not a number"));
                        }

                        arg_count = arg_count + 1;
                        if let ObjectType::CONS(cons) = restlist.value {
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
