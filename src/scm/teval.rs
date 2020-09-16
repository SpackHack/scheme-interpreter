use super::printer::display_or_print;
use super::scm_object::*;
use super::stack::*;
use std::fs::File;

use std::rc::Rc;

static mut STACK: Stack<ScmObject> = Stack::new();
static mut RETURN_STACK: Stack<ReturnFunction> = Stack::new();

static mut RETURN_VALUE: ScmObject = ScmObject::Nil;

fn pop() -> ScmObject {
    unsafe {
        match STACK.pop() {
            Some(s) => s,
            None => ScmObject::Null,
        }
    }
}

fn push(scm: ScmObject) {
    unsafe { STACK.push(scm) }
}

fn get_stack_element(index: i64) -> Option<ScmObject> {
    unsafe { STACK.remove(index) }
}

fn get_stack_size() -> i64 {
    unsafe { STACK.get_length() }
}

fn pop_re() -> Option<ReturnFunction> {
    unsafe { RETURN_STACK.pop() }
}

fn push_re(func: ReturnFunction) {
    unsafe { RETURN_STACK.push(func) }
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

pub fn eval(input: ScmObject, env: ScmObject) -> ScmObject {
    push(env);
    push(input);

    let re = trampolin(t_eval);
    return re;
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
    let env: ScmObject = pop();

    let a = expression.clone();

    match a {
        ScmObject::Symbol(_) => {
            unsafe {
                let value = Rc::get_mut_unchecked(&mut env.get_env()).get(expression);
                set_return_value(value);
            }
            return pop_re();
        }
        ScmObject::Cons(cons) => {
            push(env.clone());
            push(expression);
            push(env);
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
    let env: ScmObject = pop();

    if let ScmObject::Cons(cons) = expression {
        match &func_or_syntax {
            ScmObject::Function(_) => {
                push(env);
                push(func_or_syntax);
                push(*cons.cdr);
                return Some(ReturnFunction::new(build_in_function));
            }
            ScmObject::Syntax(_) => {
                push(env);
                push(func_or_syntax);
                push(*cons.cdr);
                return Some(ReturnFunction::new(build_in_syntax));
            }
            ScmObject::UserFunction(_) => {
                push(env);
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
    let env = pop();

    let stack_index_of_first_arg = ScmObject::Integer(get_stack_size() as i64);

    if let ScmObject::Cons(cons) = args {
        push(stack_index_of_first_arg.clone());
        push(*cons.cdr);
        push(func.clone());
        push(env.clone());

        push(env);
        push(*cons.car);
        push_re(ReturnFunction::new(build_in_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    push(stack_index_of_first_arg);
    push(func);
    push(env);
    return Some(ReturnFunction::new(build_in_function2));
}

fn build_in_function1() -> Option<ReturnFunction> {
    let env = pop();
    let func: ScmObject = pop();
    let args: ScmObject = pop();
    let stack_index_of_first_arg = pop();

    let next_argument: ScmObject = get_return_value();

    push(next_argument);

    if let ScmObject::Cons(cons) = args {
        push(stack_index_of_first_arg.clone());
        push(*cons.cdr);
        push(func.clone());
        push(env.clone());

        push(env);
        push(*cons.car);
        push_re(ReturnFunction::new(build_in_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    push(stack_index_of_first_arg);
    push(func);
    push(env);
    return Some(ReturnFunction::new(build_in_function2));
}

fn build_in_function2() -> Option<ReturnFunction> {
    let env = pop();
    let func: ScmObject = pop();
    let stack_index_of_first_arg = pop();

    let index_first_arg = stack_index_of_first_arg.get_integer();

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
                let mut sum: f64 = 0.0;
                let mut arg;
                let mut is_integer: bool = true;

                while arg_count > 0 {
                    arg = pop();
                    arg_count -= 1;
                    match arg {
                        // TODO: overflow
                        ScmObject::Integer(number) => {
                            sum += number as f64;
                        }
                        ScmObject::Float(number) => {
                            sum += number;
                            is_integer = false;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn +: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                if is_integer {
                    set_return_value(ScmObject::Integer(sum as i64));
                } else {
                    set_return_value(ScmObject::Float(sum));
                }
                return pop_re();
            }
            BuildInFunction::Minus => {
                let mut sum: f64 = 0.0;
                let mut arg;
                let mut is_integer: bool = true;

                if arg_count == 1 {
                    match pop() {
                        ScmObject::Integer(number) => {
                            set_return_value(ScmObject::Integer(number * -1))
                        }
                        ScmObject::Float(number) => {
                            set_return_value(ScmObject::Float(number * -1.0))
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn -: arg not a number",
                            )));
                            return None;
                        }
                    }
                    return pop_re();
                }

                while arg_count > 1 {
                    arg = pop();
                    arg_count -= 1;
                    match arg {
                        // TODO: overflow
                        ScmObject::Integer(number) => {
                            sum -= number as f64;
                        }
                        ScmObject::Float(number) => {
                            sum -= number;
                            is_integer = false;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn -: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                match pop() {
                    ScmObject::Integer(number) => {
                        sum += number as f64;
                    }
                    ScmObject::Float(number) => {
                        sum += number;
                        is_integer = false;
                    }
                    _ => {
                        set_return_value(ScmObject::Error(String::from("fn -: arg not a number")));
                        return None;
                    }
                }

                if is_integer {
                    set_return_value(ScmObject::Integer(sum as i64));
                } else {
                    set_return_value(ScmObject::Float(sum));
                }
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
            BuildInFunction::PrintEnv => {
                unsafe { Rc::get_mut_unchecked(&mut env.get_env()).print() }
                set_return_value(ScmObject::Void);
                return pop_re();
            }
            BuildInFunction::Times => {
                let mut product = 1.0;
                let mut arg;
                let mut is_integer: bool = true;

                while arg_count > 0 {
                    // TODO: overflow
                    arg = pop();
                    arg_count -= 1;
                    match arg {
                        ScmObject::Integer(number) => {
                            product *= number as f64;
                        }
                        ScmObject::Float(number) => {
                            is_integer = false;
                            product *= number;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn *: arg not a number",
                            )));
                            return None;
                        }
                    }
                }
                if is_integer {
                    set_return_value(ScmObject::Integer(product as i64));
                } else {
                    set_return_value(ScmObject::Float(product));
                }
                return pop_re();
            }
            BuildInFunction::Div => {
                let mut result: f64 = 0.0;
                let mut is_integer: bool = true;

                if let Some(o) = get_stack_element(index_first_arg) {
                    match o {
                        ScmObject::Integer(number) => result = number as f64,
                        ScmObject::Float(number) => {
                            is_integer = false;
                            result = number;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn /: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                while let Some(o) = get_stack_element(index_first_arg) {
                    match o {
                        ScmObject::Integer(number) => result = result / number as f64,
                        ScmObject::Float(number) => {
                            is_integer = false;
                            result = result / number;
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn /: arg not a number",
                            )));
                            return None;
                        }
                    }
                }

                if is_integer {
                    set_return_value(ScmObject::Integer(result as i64));
                } else {
                    set_return_value(ScmObject::Float(result));
                }
                return pop_re();
            }
            BuildInFunction::Cons => {
                let cdr = pop();
                let car = pop();

                set_return_value(ScmObject::new_cons(car, cdr));
                return pop_re();
            }
            BuildInFunction::Car => {
                if let ScmObject::Cons(cons) = pop() {
                    set_return_value(*cons.car);
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from("fn car: arg not a cons")));
                    return None;
                }
            }
            BuildInFunction::Cdr => {
                if let ScmObject::Cons(cons) = pop() {
                    set_return_value(*cons.cdr);
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from("fn car: arg not a cons")));
                    return None;
                }
            }
            BuildInFunction::Equal => {
                let arg2 = pop();
                let arg1 = pop();

                if arg1.equal(&arg2) {
                    set_return_value(ScmObject::True);
                } else {
                    set_return_value(ScmObject::False);
                }
                return pop_re();
            }
            BuildInFunction::Gt => {
                let arg2 = pop();
                let arg1 = pop();

                match arg1 {
                    ScmObject::Integer(number) => match arg2 {
                        ScmObject::Integer(num) => {
                            if number > num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        ScmObject::Float(num) => {
                            if number as f64 > num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        _ => {}
                    },
                    ScmObject::Float(number) => match arg2 {
                        ScmObject::Integer(num) => {
                            if number > num as f64 {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        ScmObject::Float(num) => {
                            if number as f64 > num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        _ => {}
                    },
                    _ => {}
                }
                set_return_value(ScmObject::Error(String::from("fn gt: arg not a number")));
                return None;
            }
            BuildInFunction::IsChars => {
                if let ScmObject::Chars(_) = pop() {
                    set_return_value(ScmObject::True);
                } else {
                    set_return_value(ScmObject::False);
                }
                return pop_re();
            }
            BuildInFunction::IsCons => {
                if let ScmObject::Cons(_) = pop() {
                    set_return_value(ScmObject::True);
                } else {
                    set_return_value(ScmObject::False);
                }
                return pop_re();
            }
            BuildInFunction::IsNumber => {
                match pop() {
                    ScmObject::Integer(_) => set_return_value(ScmObject::True),
                    ScmObject::Float(_) => set_return_value(ScmObject::True),
                    _ => set_return_value(ScmObject::False),
                }
                return pop_re();
            }
            BuildInFunction::IsInteger => {
                match pop() {
                    ScmObject::Integer(_) => set_return_value(ScmObject::True),
                    _ => set_return_value(ScmObject::False),
                }
                return pop_re();
            }
            BuildInFunction::IsFloat => {
                match pop() {
                    ScmObject::Float(_) => set_return_value(ScmObject::True),
                    _ => set_return_value(ScmObject::False),
                }
                return pop_re();
            }
            BuildInFunction::IsFunction => {
                match pop() {
                    ScmObject::Function(_) => set_return_value(ScmObject::True),
                    ScmObject::UserFunction(_) => set_return_value(ScmObject::True),
                    _ => set_return_value(ScmObject::False),
                }
                return pop_re();
            }
            BuildInFunction::IsSymbole => {
                if let ScmObject::Symbol(_) = pop() {
                    set_return_value(ScmObject::True);
                } else {
                    set_return_value(ScmObject::False);
                }
                return pop_re();
            }
            BuildInFunction::IsNull => {
                if let ScmObject::Null = pop() {
                    set_return_value(ScmObject::True);
                } else {
                    set_return_value(ScmObject::False);
                }
                return pop_re();
            }
            BuildInFunction::StringLength => {
                if let ScmObject::Chars(s) = pop() {
                    set_return_value(ScmObject::Integer(s.len() as i64));
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "fn String Length: arg not a String",
                    )));
                    return None;
                }
            }
            BuildInFunction::EqualString => {
                let arg2 = pop();
                let arg1 = pop();

                if let ScmObject::Chars(s2) = arg2 {
                    if let ScmObject::Chars(s1) = arg1 {
                        if s1 == s2 {
                            set_return_value(ScmObject::True);
                        } else {
                            set_return_value(ScmObject::False);
                        }
                        return pop_re();
                    } else {
                        set_return_value(ScmObject::Error(String::from(
                            "fn String=?: arg not a String",
                        )));
                        return None;
                    }
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "fn String=?: arg not a String",
                    )));
                    return None;
                }
            }
            BuildInFunction::StringAppend => {
                let mut result: String = String::new();

                while let Some(o) = get_stack_element(index_first_arg) {
                    match o {
                        ScmObject::Chars(s) => {
                            result.push_str(&s);
                        }
                        _ => {
                            set_return_value(ScmObject::Error(String::from(
                                "fn String append: arg not a string",
                            )));
                            return None;
                        }
                    }
                }
                set_return_value(ScmObject::Chars(result));
                return pop_re();
            }
            BuildInFunction::Length => {
                let mut arg = pop();
                let mut result = 0;
                while let ScmObject::Cons(c) = arg {
                    arg = *c.cdr;
                    result += 1;
                }

                set_return_value(ScmObject::Integer(result));
                return pop_re();
            }
            BuildInFunction::Append => {
                let arg2 = pop();
                let mut arg1 = pop();
                let mut arg = &mut arg1;

                if let ScmObject::Null = arg {
                    set_return_value(arg2);
                    return pop_re();
                } else {
                    while let ScmObject::Cons(c) = arg {
                        if let ScmObject::Nil = *c.cdr {
                            if let ScmObject::Null = arg2 {
                                set_return_value(arg1);
                                return pop_re();
                            } else {
                                *c.cdr = arg2;
                                set_return_value(arg1);
                                return pop_re();
                            }
                        } else {
                            arg = &mut *c.cdr;
                        }
                    }
                }
            }
            BuildInFunction::EqualNumber => {
                let arg2 = pop();
                let arg1 = pop();

                match arg1 {
                    ScmObject::Integer(number) => match arg2 {
                        ScmObject::Integer(num) => {
                            if number == num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        ScmObject::Float(num) => {
                            if number as f64 == num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        _ => {}
                    },
                    ScmObject::Float(number) => match arg2 {
                        ScmObject::Integer(num) => {
                            if number == num as f64 {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        ScmObject::Float(num) => {
                            if number == num {
                                set_return_value(ScmObject::True);
                            } else {
                                set_return_value(ScmObject::False);
                            }
                            return pop_re();
                        }
                        _ => {}
                    },
                    _ => {}
                }

                set_return_value(ScmObject::Error(String::from("fn =: arg not a number")));
                return None;
            }
            BuildInFunction::FnBody => {
                let arg = pop();

                if let ScmObject::UserFunction(func) = arg {
                    set_return_value(*func.body_list);
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "fn fnBody: arg not a user function",
                    )));
                    return None;
                }
            }
            BuildInFunction::FnArg => {
                let arg = pop();

                if let ScmObject::UserFunction(func) = arg {
                    set_return_value(*func.arg_list);
                    return pop_re();
                } else {
                    set_return_value(ScmObject::Error(String::from(
                        "fn fnArg: arg not a user function",
                    )));
                    return None;
                }
            }
            BuildInFunction::List => {
                let mut rest = ScmObject::Nil;
                let mut arg;
                while arg_count > 0 {
                    arg = pop();
                    arg_count -= 1;
                    rest = ScmObject::new_cons(arg, rest);
                }
                set_return_value(rest);
                return pop_re();
            }
            BuildInFunction::Load => {
                let scm_file_name = pop();

                let file_name: String;
                let mut input_stream: ScmObject;
                let expression: ScmObject;

                if let ScmObject::Chars(c) = scm_file_name {
                    file_name = c;
                } else {
                    set_return_value(ScmObject::Error(String::from("fn load: arg not a String")));
                    return None;
                }

                match File::open(file_name) {
                    Ok(file) => {
                        input_stream = ScmObject::new_stream_file(file);
                    }
                    Err(_e) => {
                        set_return_value(ScmObject::Error(String::from(
                            "fn load: con not find File",
                        )));
                        return None;
                    }
                }

                expression = super::reader::scm_read(&mut input_stream);

                if let ScmObject::EndOfFile = expression {
                    set_return_value(ScmObject::Void);
                    return pop_re();
                } else {
                    // TODO set Top env

                    push(env.clone());
                    push(input_stream);
                    push(env);
                    push(expression);
                    push_re(ReturnFunction::new(t_read));
                    return Some(ReturnFunction::new(t_eval));
                }
            }
        }
    }
    set_return_value(ScmObject::Error(String::from("fn: is not a function")));
    return None;
}

fn t_print(is_print: bool, stack_index_of_first_arg: i64) {
    while let Some(s) = get_stack_element(stack_index_of_first_arg) {
        display_or_print(s, is_print);
    }
    println!();
}

fn t_read() -> Option<ReturnFunction> {
    let mut input_stream = pop();
    let env = pop();

    let expression = super::reader::scm_read(&mut input_stream);

    if let ScmObject::EndOfFile = expression {
        set_return_value(ScmObject::Void);
        return pop_re();
    } else {
        // TODO set Top env
        push(env.clone());
        push(input_stream);
        push(env);
        push(expression);
        push_re(ReturnFunction::new(t_read));
        return Some(ReturnFunction::new(t_eval));
    }
}

fn build_in_syntax() -> Option<ReturnFunction> {
    let mut args: ScmObject = pop();
    let syntax: ScmObject = pop();
    let env = pop();

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

                push(env.clone());
                push(synonym);
                push(env);
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

                push(env.clone());
                push(synonym);
                push(env);
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

                set_return_value(ScmObject::new_user_fn(None, arglist, body, env.get_env()));
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

                push(env.clone());
                if let ScmObject::Nil = args {
                    push(next_expression);
                    return Some(ReturnFunction::new(t_eval));
                }
                push(args);
                push(env);
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
                push(env.clone());

                push(env);
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
    let env = pop();

    unsafe {
        Rc::get_mut_unchecked(&mut env.get_env()).define(synonym, &get_return_value());
    }

    set_return_value(ScmObject::Void);
    return pop_re();
}

fn t_set() -> Option<ReturnFunction> {
    let synonym = pop();
    let env = pop();

    unsafe {
        set_return_value(
            Rc::get_mut_unchecked(&mut env.get_env()).set(synonym, &get_return_value()),
        );
    }
    return pop_re();
}

fn t_begin() -> Option<ReturnFunction> {
    let mut args = pop();
    let env = pop();

    let expression: ScmObject;

    if let ScmObject::Cons(cons) = args {
        expression = *cons.car;
        args = *cons.cdr;
    } else {
        panic!("Begin has no args");
    }

    if let ScmObject::Nil = args {
        push(env);
        push(expression);
        return Some(ReturnFunction::new(t_eval));
    }
    push(env.clone());
    push(args);

    eval(expression.clone(), env.clone());

    push(env);
    push(expression);
    push_re(ReturnFunction::new(t_begin));
    return Some(ReturnFunction::new(t_eval));
}

fn t_if() -> Option<ReturnFunction> {
    let env = pop();
    let false_expression: ScmObject = pop();
    let true_expression: ScmObject = pop();

    push(env);
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
    let mut env = pop();

    if let ScmObject::Cons(cons) = args {
        let start_index = ScmObject::Integer(get_stack_size() as i64);

        push(start_index);
        push(function);
        push(*cons.cdr);
        push(env.clone());
        push(env);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    if let ScmObject::UserFunction(func) = function {
        env = ScmObject::new_env();
        unsafe {
            Rc::get_mut_unchecked(&mut env.get_env()).set_parent_env(func.home_environment);
        }

        if let ScmObject::Nil = *func.arg_list {
            match *func.body_list {
                ScmObject::Cons(cons) => {
                    if let ScmObject::Nil = *cons.cdr {
                        push(env);
                        push(*cons.car);
                        return Some(ReturnFunction::new(t_eval));
                    }
                    push(env.clone());
                    push(*cons.cdr);
                    push(env);
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
    let mut env = pop();
    let args = pop();
    let function = pop();
    let start_index = pop();

    push(get_return_value());

    if let ScmObject::Cons(cons) = args {
        push(start_index);
        push(function);
        push(*cons.cdr);
        push(env.clone());
        push(env);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function1));
        return Some(ReturnFunction::new(t_eval));
    }

    if let ScmObject::UserFunction(func) = function {
        env = ScmObject::new_env();
        unsafe {
            Rc::get_mut_unchecked(&mut env.get_env()).set_parent_env(func.home_environment);
        }

        let stack_index_of_arg = start_index.get_integer();
        let mut arg_names = *func.arg_list;

        while let ScmObject::Cons(cons) = arg_names {
            if let Some(s) = get_stack_element(stack_index_of_arg) {
                unsafe {
                    Rc::get_mut_unchecked(&mut env.get_env()).define(*cons.car, &s);
                }
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
            if let ScmObject::Nil = *cons.cdr {
                push(env);
                push(*cons.car);
                return Some(ReturnFunction::new(t_eval));
            }
            push(env.clone());
            push(*cons.cdr);
            push(env);
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
    let env = pop();

    if let ScmObject::Cons(cons) = body {
        if let ScmObject::Nil = *cons.cdr {
            push(env);
            push(*cons.car);
            return Some(ReturnFunction::new(t_eval));
        }
        push(env.clone());
        push(*cons.cdr);
        push(env);
        push(*cons.car);
        push_re(ReturnFunction::new(t_user_function2));
        return Some(ReturnFunction::new(t_eval));
    }
    set_return_value(ScmObject::Error(String::from("user fn: body is empty")));
    return None;
}
