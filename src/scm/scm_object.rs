use super::environment::ScmEnvironment;
use super::stream::{ScmStream, StreamType};
use std::collections::HashMap;
use std::fs::File;
use std::io::stdin;
use std::rc::Rc;

#[derive(Clone)]
pub enum ScmObject {
    Error(String),
    Integer(i64),
    Float(f64),
    Chars(String),
    Cons(Cons),
    Nil, // end of list
    Symbol(String),
    Function(ScmBuildInFunction),
    Syntax(ScmBuildInSyntax),
    UserFunction(ScmUserFunction),
    EndOfFile,
    Null,
    Void,
    True,
    False,
    Env(Rc<ScmEnvironment>),
    Stream(ScmStream),
}

pub enum NumberType {
    Integer(String),
    Float(String),
}

#[derive(Clone)]
pub struct Cons {
    pub car: Box<ScmObject>,
    pub cdr: Box<ScmObject>,
}

#[derive(Clone, PartialEq)]
pub struct ScmBuildInFunction {
    pub tag: BuildInFunction,
    pub name: String,
    pub num_args: Option<i64>,
}

pub enum NumArgs {
    Unlimited = -1,
}

#[derive(Clone, PartialEq)]
pub enum BuildInFunction {
    Plus,
    Minus,
    Display,
    Print,
    PrintEnv,
    Times,
    Div,
    Cons,
    Car,
    Cdr,
    Equal,
    Gt,
    IsChars,
    IsCons,
    IsNumber,
    IsInteger,
    IsFloat,
    IsFunction,
    IsSymbole,
    IsNull,
    StringLength,
    EqualString,
    StringAppend,
    Length,
    Append,
    EqualNumber,
    FnBody,
    FnArg,
    List,
    Load,
    Exit,
}

#[derive(Clone, PartialEq)]
pub struct ScmBuildInSyntax {
    pub tag: BuildInSyntax,
    pub name: String,
    pub num_args: i64,
}

#[derive(Clone, PartialEq)]
pub enum BuildInSyntax {
    Quote,
    Lambda,
    Define,
    If,
    Set,
    Begin,
}

#[derive(Clone)]
pub struct ScmUserFunction {
    pub name: Option<String>,
    pub arg_list: Box<ScmObject>,
    pub body_list: Box<ScmObject>,
    pub home_environment: Rc<ScmEnvironment>,
}

impl ScmObject {
    pub fn new_cons(car: ScmObject, cdr: ScmObject) -> Self {
        ScmObject::Cons(Cons {
            car: Box::new(car),
            cdr: Box::new(cdr),
        })
    }

    pub fn new_fn(tag: BuildInFunction, name: String, num_of_args: i64) -> Self {
        let args: Option<i64>;
        if num_of_args != -1 {
            args = Some(num_of_args);
        } else {
            args = None;
        }

        ScmObject::Function(ScmBuildInFunction {
            tag: tag,
            name: name,
            num_args: args,
        })
    }

    pub fn new_syntax(tag: BuildInSyntax, name: String, num_of_args: i64) -> Self {
        ScmObject::Syntax(ScmBuildInSyntax {
            tag: tag,
            name: name,
            num_args: num_of_args,
        })
    }

    pub fn new_user_fn(
        name: Option<String>,
        arg_list: ScmObject,
        body_list: ScmObject,
        home_environment: Rc<ScmEnvironment>,
    ) -> Self {
        ScmObject::UserFunction(ScmUserFunction {
            name: name,
            arg_list: Box::from(arg_list),
            body_list: Box::from(body_list),
            home_environment: Rc::clone(&home_environment),
        })
    }

    pub fn new_env() -> Self {
        ScmObject::Env(Rc::new(ScmEnvironment {
            bindings: HashMap::new(),
            parent_env: None,
        }))
    }

    pub fn new_stream() -> Self {
        ScmObject::Stream(ScmStream {
            stream_type: StreamType::STDIN(Rc::new(stdin())),
            read_char: vec![],
        })
    }

    pub fn new_stream_file(file: File) -> Self {
        ScmObject::Stream(ScmStream {
            stream_type: StreamType::FILE(Rc::new(file)),
            read_char: vec![],
        })
    }

    pub fn get_integer(&self) -> i64 {
        if let ScmObject::Integer(n) = self {
            return *n;
        }
        panic!("get Integer of not a Integer");
    }

    pub fn get_env(&self) -> Rc<ScmEnvironment> {
        if let ScmObject::Env(env) = self {
            return env.clone();
        }
        panic!("get Env of not env");
    }

    pub fn equal(&self, scm: &ScmObject) -> bool {
        match self {
            ScmObject::Error(_) => {
                return false;
            }
            ScmObject::Integer(number) => {
                if let ScmObject::Integer(num) = scm {
                    if number == num {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::Float(number) => {
                if let ScmObject::Float(num) = scm {
                    if number == num {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::Chars(chars) => {
                if let ScmObject::Chars(ch) = scm {
                    if chars == ch {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::Cons(cons) => {
                if let ScmObject::Cons(co) = scm {
                    if !co.car.equal(&*cons.car) {
                        return false;
                    }
                    if let ScmObject::Nil = *cons.cdr {
                        if let ScmObject::Nil = *co.cdr {
                            return true;
                        }
                        return false;
                    }
                    if let ScmObject::Nil = *co.cdr {
                        return false;
                    }
                    return cons.cdr.equal(&*co.cdr);
                }
                return false;
            }
            ScmObject::Nil => {
                if let ScmObject::Null = scm {
                    return true;
                }
                return false;
            }
            ScmObject::Symbol(symbole) => {
                if let ScmObject::Symbol(s) = &scm {
                    if symbole == s {
                        return true;
                    }
                }
                false
            }
            ScmObject::Function(function) => {
                if let ScmObject::Function(func) = scm {
                    if function == func {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::Syntax(syntax) => {
                if let ScmObject::Syntax(sy) = scm {
                    if syntax == sy {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::UserFunction(function) => {
                if let ScmObject::UserFunction(func) = scm {
                    if function.arg_list.equal(&func.arg_list)
                        && function.body_list.equal(&func.body_list)
                    {
                        return true;
                    }
                }
                return false;
            }
            ScmObject::EndOfFile => {
                if let ScmObject::EndOfFile = scm {
                    return true;
                }
                return false;
            }
            ScmObject::Null => {
                if let ScmObject::Null = scm {
                    return true;
                }
                return false;
            }
            ScmObject::Void => {
                if let ScmObject::Void = scm {
                    return true;
                }
                return false;
            }
            ScmObject::True => {
                if let ScmObject::True = scm {
                    return true;
                }
                false
            }
            ScmObject::False => {
                if let ScmObject::False = scm {
                    return true;
                }
                false
            }
            ScmObject::Env(_) => {
                return false;
            }
            ScmObject::Stream(_) => {
                return false;
            }
        }
    }
}
