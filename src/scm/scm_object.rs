use super::environment::ScmEnvironment;

#[derive(Clone)]
pub enum ScmObject {
    Error(String),
    Number(i64),
    Chars(String),
    Cons(Cons),
    Nil, // end of list
    Symbol(String),
    Function(ScmBuildInFunction),
    Syntax(ScmBuildInSyntax),
    UserFunction(ScmUserFunction),
    EndOfFile,
    None,
    Null,
    Void,
    True,
    False,
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
    Cons,
    Car,
    Cdr,
    Equal,
    Gt,
    IsChars,
    IsCons,
    IsNumber,
    IsFunction,
    IsSyntax,
    IsUserFunctions,
    EqualNumber,
    FnBody,
    FnArg,
    List,
    Load,
    Open,
    Close,
    Read,
    ReadChar,
    ReadLine,
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
    pub home_environment: Box<ScmEnvironment>,
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
        home_environment: ScmEnvironment,
    ) -> Self {
        ScmObject::UserFunction(ScmUserFunction {
            name: name,
            arg_list: Box::from(arg_list),
            body_list: Box::from(body_list),
            home_environment: Box::from(home_environment),
        })
    }

    pub fn get_number(&self) -> i64 {
        if let ScmObject::Number(n) = self {
            return *n;
        }
        panic!("get Number of not a number");
    }

    pub fn equal(&self, scm: &ScmObject) -> bool {
        match self {
            ScmObject::Error(_) => {
                return false;
            }
            ScmObject::Number(number) => {
                if let ScmObject::Number(num) = scm {
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
                //TODO: cons equal
                if let ScmObject::Cons(co) = scm {
                    return true;
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
                //TODO: user function equal
                if let ScmObject::UserFunction(func) = scm {
                    return true;
                }
                return false;
            }
            ScmObject::EndOfFile => {
                if let ScmObject::EndOfFile = scm {
                    return true;
                }
                return false;
            }
            ScmObject::None => {
                if let ScmObject::None = scm {
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
        }
    }
}
