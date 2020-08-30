use super::environment::*;
use super::printer::display_or_print;
use super::scm_object::*;
use super::stack::*;

static mut STACK: Stack<ScmObject> = Stack::new(100);
static mut RETURN_STACK: Stack<ReturnFunction> = Stack::new(100);
static mut ENV_STACK: Stack<ScmEnvironment> = Stack::new(100);
static mut ENV_COUNTER: Vec<i64> = vec![];

static mut RETURN_VALUE: ScmObject = ScmObject::Nil;

fn pop() -> ScmObject {
    unsafe {
        match STACK.pop() {
            Some(s) => s,
            None => ScmObject::None,
        }
    }
}

fn push(scm: ScmObject) {
    unsafe { STACK.push(scm) }
}

fn get_stack_element(index: i64) -> Option<ScmObject> {
    unsafe { STACK.get(index) }
}

fn get_stack_size() -> usize {
    unsafe { STACK.get_length() }
}

fn pop_re() -> Option<ReturnFunction> {
    unsafe { RETURN_STACK.pop() }
}

fn push_re(func: ReturnFunction) {
    unsafe { RETURN_STACK.push(func) }
}

fn pop_env() -> ScmEnvironment {
    let re: ScmEnvironment;
    unsafe {
        if let Some(env_count) = ENV_COUNTER.pop() {
            match ENV_STACK.pop() {
                Some(env) => {
                    re = env;
                }
                None => panic!("no more env"),
            }

            if env_count != 0 {
                ENV_COUNTER.push(env_count - 1);
                ENV_STACK.push(re.clone());
            }
        } else {
            panic!("no more env");
        }
    }
    re
}

fn push_env(env: &ScmEnvironment, new: bool) {
    unsafe {
        if new {
            ENV_STACK.push(env.clone());
            ENV_COUNTER.push(0);
        } else {
            if let Some(env_count) = ENV_COUNTER.pop() {
                update_env(env.clone());
                ENV_COUNTER.push(env_count + 1);
            } else {
                ENV_STACK.push(env.clone());
                ENV_COUNTER.push(0);
            }
        }
    }
}

fn update_env(env: ScmEnvironment) {
    unsafe {
        if let Some(_) = ENV_STACK.pop() {
            ENV_STACK.push(env);
        }
    }
}

fn set_return_value(value: ScmObject) {
    unsafe { RETURN_VALUE = value }
}

fn get_return_value() -> ScmObject {
    unsafe { RETURN_VALUE.clone() }
}

pub struct ReturnFunction {
    pub func: fn() -> Option<ReturnFunction>,
}

impl ReturnFunction {
    pub fn new(func: fn() -> Option<ReturnFunction>) -> Self {
        ReturnFunction { func: func }
    }
}

pub fn eval(input: ScmObject, env: ScmEnvironment) -> (ScmObject, ScmEnvironment) {
    push_env(&env, false); // for return env
    push_env(&env, false);
    push(input);
    return (trampolin(t_eval), pop_env());
}

fn trampolin(function: fn() -> Option<ReturnFunction>) -> ScmObject {
    let mut next_function_ptr: Option<ReturnFunction> = Some(ReturnFunction { func: function });
    while let Some(f) = next_function_ptr {
        next_function_ptr = (f.func)();
    }

    return get_return_value();
}

fn t_eval() -> Option<ReturnFunction> {
    let expression: ScmObject = pop();
    let mut env: ScmEnvironment = pop_env();

    let a = expression.clone();

    match a {
        ScmObject::Symbol(_) => {
            set_return_value(env.get(expression));
            return pop_re();
        }
        ScmObject::Cons(cons) => {
            push_env(&env, false);
            push(expression);
            push_env(&env, false);
            push(*cons.car);
            push_re(ReturnFunction::new(t_eval2));
            return Some(ReturnFunction::new(t_eval));
        }
        _ => {}
    }
    set_return_value(expression);
    return pop_re();
}

fn t_eval2() -> Option<ReturnFunction> {
    let func_or_syntax: ScmObject = get_return_value();
    let expression: ScmObject = pop();
    let env: ScmEnvironment = pop_env();

    if let ScmObject::Cons(cons) = expression {
        match &func_or_syntax {
            ScmObject::Function(_) => {
                push_env(&env, false);
                push(func_or_syntax);
                push(*cons.cdr);
                return Some(ReturnFunction::new(build_in_function));
            }
            ScmObject::Syntax(_) => {
                push_env(&env, false);
                push(func_or_syntax);
                push(*cons.cdr);
                return Some(ReturnFunction::new(build_in_syntax));
            }
            ScmObject::UserFunction(_) => {
                push_env(&env, false);
                push(func_or_syntax);
                push(*cons.cdr);
                return Some(ReturnFunction::new(t_user_function));
            }
            _ => {}
        }
    }
    set_return_value(ScmObject::Error(String::from("Not a valid function")));
    return None;
}

fn build_in_function() -> Option<ReturnFunction> {
    let args: ScmObject = pop();
    let func: ScmObject = pop();
    let env = pop_env();

    let stack_index_of_first_arg = ScmObject::Number(get_stack_size() as i64);

    if let ScmObject::Cons(cons) = args {
        push(stack_index_of_first_arg.clone());
        push(*cons.cdr);
        push(func.clone());
        push_env(&env, false);

        push_env(&env, false);
        push(*cons.car);
        push_re(ReturnFunction::new(build_in_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    push(stack_index_of_first_arg);
    push(func);
    push_env(&env, false);
    return Some(ReturnFunction::new(build_in_function2));
}

fn build_in_function1() -> Option<ReturnFunction> {
    let env = pop_env();
    let func: ScmObject = pop();
    let args: ScmObject = pop();
    let stack_index_of_first_arg = pop();

    let next_argument: ScmObject = get_return_value();

    push(next_argument);

    if let ScmObject::Cons(cons) = args {
        push(stack_index_of_first_arg.clone());
        push(*cons.cdr);
        push(func.clone());
        push_env(&env, false);

        push_env(&env, false);
        push(*cons.car);
        push_re(ReturnFunction::new(build_in_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    push(stack_index_of_first_arg);
    push(func);
    push_env(&env, false);
    return Some(ReturnFunction::new(build_in_function2));
}

fn build_in_function2() -> Option<ReturnFunction> {
    let env = pop_env();
    let func: ScmObject = pop();
    let stack_index_of_first_arg = pop();

    let index_first_arg = stack_index_of_first_arg.get_number();

    let mut arg_count = get_stack_size() as i64 - index_first_arg;

    if let ScmObject::Function(func) = func {
        if let Some(num_args) = func.num_args {
            if num_args != arg_count {
                set_return_value(ScmObject::Error(String::from(
                    "fn: not the right amount of arguments.",
                )));
                return None;
            }
        }

        match func.tag {
            BuildInFunction::Plus => {
                let mut sum: i64 = 0;
                let mut arg;
                while arg_count > 0 {
                    arg = pop();
                    arg_count -= 1;
                    match arg {
                        // TODO: overflow
                        ScmObject::Number(number) => {
                            sum += number;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn +: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                set_return_value(ScmObject::Number(sum));
                return pop_re();
            }
            BuildInFunction::Minus => {
                let mut sum: i64 = 0;
                let mut arg;
                while arg_count > 1 {
                    arg = pop();
                    arg_count -= 1;
                    match arg {
                        // TODO: overflow
                        ScmObject::Number(number) => {
                            sum -= number;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn -: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                arg = pop();

                if let ScmObject::Number(number) = arg {
                    sum += number;
                } else {
                    set_return_value(ScmObject::Error(String::from("fn -: arg not a number")));
                    return None;
                }

                set_return_value(ScmObject::Number(sum));
                return pop_re();
            }
            BuildInFunction::Display => {
                t_print(false, index_first_arg);
                set_return_value(ScmObject::Void);
                return pop_re();
            }
            BuildInFunction::Print => {
                t_print(true, index_first_arg);
                set_return_value(ScmObject::Void);
                return pop_re();
            }
            _ => {
                set_return_value(ScmObject::Error(String::from(
                    "fn: is not impl",
                )));
                return None;
            }
        }
    }
    set_return_value(ScmObject::Error(String::from(
        "fn: is not a function",
    )));
    return None;
}

fn t_print(is_print: bool, stack_index_of_first_arg: i64) {
    while let Some(s) = get_stack_element(stack_index_of_first_arg) {
        display_or_print(s, is_print);
    }
}

fn build_in_syntax() -> Option<ReturnFunction> {
    let mut args: ScmObject = pop();
    let syntax: ScmObject = pop();
    let env = pop_env();

    if let ScmObject::Syntax(syntax) = syntax {
        match syntax.tag {
            BuildInSyntax::Quote => {
                let argument: ScmObject;

                if let ScmObject::Cons(cons) = args {
                    argument = *cons.car;
                    args = *cons.cdr
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "quote: need 1 argument but has 0",
                    )));
                    return None;
                }

                if let ScmObject::Nil = args {
                    set_return_value(argument);
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "quote: only 1 argument allowed",
                    )));
                    return None;
                }
            }
            BuildInSyntax::Define => {
                let synonym: ScmObject;
                let value: ScmObject;

                if let ScmObject::Cons(cons) = args {
                    synonym = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "define: need 2 argument but has 0",
                    )));
                    return None;
                }

                if let ScmObject::Cons(cons) = args {
                    value = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "define: need 2 argument but has 1",
                    )));
                    return None;
                }

                if let ScmObject::Symbol(_) = &synonym {
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "define: synonym is not a symbol",
                    )));
                    return None;
                }

                if let ScmObject::Nil = args {
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "define: need exactly 2 argument but has more",
                    )));
                    return None;
                }

                push_env(&env, false);
                push(synonym);
                push_env(&env, false);
                push(value);
                push_re(ReturnFunction::new(t_define));
                return Some(ReturnFunction::new(t_eval));
            }
            BuildInSyntax::Set => {
                let synonym: ScmObject;
                let value: ScmObject;

                if let ScmObject::Cons(cons) = args {
                    synonym = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "set: need 2 argument but has 0",
                    )));
                    return None;
                }

                if let ScmObject::Cons(cons) = args {
                    value = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "set: need 2 argument but has 1",
                    )));
                    return None;
                }

                if let ScmObject::Symbol(_) = &synonym {
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "set: synonym is not a symbol",
                    )));
                    return None;
                }

                if let ScmObject::Nil = args {
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "set: need exactly 2 argument but has more",
                    )));
                    return None;
                }

                push_env(&env, false);
                push(synonym);
                push_env(&env, false);
                push(value);
                push_re(ReturnFunction::new(t_set));
                return Some(ReturnFunction::new(t_eval));
            }
            BuildInSyntax::Lambda => {
                let arglist: ScmObject;
                let body: ScmObject;

                if let ScmObject::Cons(cons) = args {
                    arglist = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "lambda: need at least 2 argument, but has 0",
                    )));
                    return None;
                }

                if let ScmObject::Nil = args {
                    set_return_value(ScmObject::Error(String::from(
                        "lambda: need at least 2 argument, but has 1",
                    )));
                    return None;
                } else {
                    body = args;
                }

                set_return_value(ScmObject::new_user_fn(None, arglist, body, env));
                return pop_re();
            }
            BuildInSyntax::Begin => {
                let next_expression: ScmObject;

                if let ScmObject::Nil = &args {
                    set_return_value(ScmObject::Void);
                    return pop_re();
                }
                if let ScmObject::Cons(cons) = args {
                    next_expression = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "begin: need at least 1 argument but has 0",
                    )));
                    return None;
                }

                push_env(&env, false);
                if let ScmObject::Nil = args {
                    push(next_expression);
                    return Some(ReturnFunction::new(t_eval));
                }
                push(args);
                push_env(&env, false);
                push(next_expression);
                push_re(ReturnFunction::new(t_begin));
                return Some(ReturnFunction::new(t_eval));
            }
            BuildInSyntax::If => {
                let condition: ScmObject;
                let true_expression: ScmObject;
                let false_expression: ScmObject;

                if let ScmObject::Cons(cons) = args {
                    condition = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "if: need 3 argument but has 0",
                    )));
                    return None;
                }
                if let ScmObject::Cons(cons) = args {
                    true_expression = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "if: need 3 argument but has 1",
                    )));
                    return None;
                }
                if let ScmObject::Cons(cons) = args {
                    false_expression = *cons.car;
                    args = *cons.cdr;
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "if: need 3 argument but has 2",
                    )));
                    return None;
                }

                if let ScmObject::Nil = args {
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "if: need exactly 3 argument but has more",
                    )));
                    return None;
                }

                push(true_expression);
                push(false_expression);
                push_env(&env, false);
                push_env(&env, false);
                push(condition);
                push_re(ReturnFunction::new(t_if));
                return Some(ReturnFunction::new(t_eval));
            }
        }
    }

    return None;
}

fn t_define() -> Option<ReturnFunction> {
    let synonym = pop();
    let mut env = pop_env();

    env.define(synonym, &get_return_value());

    update_env(env);

    set_return_value(ScmObject::Void);
    return pop_re();
}

fn t_set() -> Option<ReturnFunction> {
    let synonym = pop();
    let mut env = pop_env();

    env.set(synonym, get_return_value());

    update_env(env);

    set_return_value(ScmObject::Void);
    return pop_re();
}

fn t_begin() -> Option<ReturnFunction> {
    let mut args = pop();
    let env = pop_env();

    let expression: ScmObject;

    if let ScmObject::Cons(cons) = args {
        expression = *cons.car;
        args = *cons.cdr;
    } else {
        panic!("Begin has no args");
    }

    if let ScmObject::Nil = args {
        push_env(&env, false);
        push(expression);
        return Some(ReturnFunction::new(t_eval));
    }
    push_env(&env, false);
    push(args);

    let re = eval(expression.clone(), env);
    push_env(&re.1, false);
    push(expression);
    push_re(ReturnFunction::new(t_begin));
    return Some(ReturnFunction::new(t_eval));
}

fn t_if() -> Option<ReturnFunction> {
    let false_expression: ScmObject = pop();
    let true_expression: ScmObject = pop();

    if get_return_value().equal(&ScmObject::True) {
        push(true_expression);
    } else {
        push(false_expression);
    }
    return Some(ReturnFunction::new(t_eval));
}

fn t_user_function() -> Option<ReturnFunction> {
    let args = pop();
    let function = pop();
    let env = pop_env();

    if let ScmObject::Cons(cons) = args {
        let start_index = ScmObject::Number(get_stack_size() as i64);

        push(start_index);
        push(function);
        push(*cons.cdr);
        push_env(&env, false);
        push_env(&env, false);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    let mut new_env = ScmEnvironment::new();
    new_env.set_parent_env(&env);

    if let ScmObject::UserFunction(func) = function {
        if let ScmObject::Nil = *func.arg_list {
            match *func.body_list {
                ScmObject::Cons(cons) => {
                    if let ScmObject::Nil = *cons.cdr {
                        push_env(&new_env, true);
                        push(*cons.car);
                        return Some(ReturnFunction::new(t_eval));
                    }
                    push_env(&new_env, true);
                    push(*cons.cdr);
                    push_env(&new_env, false);
                    push(*cons.car);
                    push_re(ReturnFunction::new(t_user_function2));
                    return Some(ReturnFunction::new(t_eval));
                }
                _ => {
                    set_return_value(ScmObject::Error(String::from("user fn: body is empty")));
                    return None;
                }
            }
        } else {
            set_return_value(ScmObject::Error(String::from("user fn: expects arguments")));
            return None;
        }
    }

    return None;
}

fn t_user_function1() -> Option<ReturnFunction> {
    let env = pop_env();
    let args = pop();
    let function = pop();
    let start_index = pop();

    push(get_return_value());

    if let ScmObject::Cons(cons) = args {
        push(start_index);
        push(function);
        push(*cons.cdr);
        push_env(&env, false);
        push_env(&env, false);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    let mut new_env = ScmEnvironment::new();
    new_env.set_parent_env(&env);

    if let ScmObject::UserFunction(func) = function {
        let stack_index_of_arg = start_index.get_number();
        let mut arg_names = *func.arg_list;

        while let ScmObject::Cons(cons) = arg_names {
            if let Some(s) = get_stack_element(stack_index_of_arg) {
                new_env.define(*cons.car, &s);
                arg_names = *cons.cdr;
            } else {
                set_return_value(ScmObject::Error(String::from(
                    "user fn: not enough Arguments",
                )));
                return None;
            }
        }
        if stack_index_of_arg < get_stack_size() as i64 {
            set_return_value(ScmObject::Error(String::from("user fn: to many Arguments")));
            return None;
        }
        if let ScmObject::Nil = *func.body_list {
            set_return_value(ScmObject::Error(String::from("user fn: body is empty")));
            return None;
        }

        if let ScmObject::Cons(cons) = *func.body_list {
            push_env(&new_env, true);
            if let ScmObject::Nil = *cons.cdr {
                push(*cons.car);
                return Some(ReturnFunction::new(t_eval));
            }
            push(*cons.cdr);
            push_env(&new_env, false);
            push(*cons.car);
            push_re(ReturnFunction::new(t_user_function2));
            return Some(ReturnFunction::new(t_eval));
        } else {
        }
    }
    set_return_value(ScmObject::Error(String::from("user fn: is not a function")));
    return None;
}

fn t_user_function2() -> Option<ReturnFunction> {
    let body = pop();
    let env = pop_env();

    if let ScmObject::Cons(cons) = body {
        if let ScmObject::Nil = *cons.cdr {
            push_env(&env, false);
            push(*cons.car);
            return Some(ReturnFunction::new(t_eval));
        }
        push_env(&env, false);
        push(*cons.cdr);
        push_env(&env, false);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function2));
        return Some(ReturnFunction::new(t_eval));
    }
    set_return_value(ScmObject::Error(String::from("user fn: body is empty")));
    return None;
}
