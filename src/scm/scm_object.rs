use super::environment::ScmEnvironment;

#[derive(Clone)]
pub enum ScmObject {
    ERROR(String),
    NUMBER(i64),
    STRING(String),
    CONS(Cons),
    NIL, // end of list
    SYMBOL(String),
    TRUE,
    FALSE,
    NULL,
    Void,
    FN(ScmBuildInFunction),
    Syntax(ScmBuildInSyntax),
    USERFN(ScmUserFunction),
    EOF,
    None,
}

#[derive(Clone)]
pub struct Cons {
    pub car: Box<ScmObject>,
    pub cdr: Box<ScmObject>,
}

#[derive(Clone)]
pub struct ScmBuildInFunction {
    pub tag: BuildInFunction,
    pub name: String,
    pub num_args: Option<i64>,
}

pub enum NumArgs {
    Unlimited = -1,
}

#[derive(Clone)]
pub enum BuildInFunction {
    Plus,
    Minus,
    Display,
    Print,
    // FN_PLUS,
    // FN_MINUS,
    // FN_DISPLAY,
    // FN_PRINT,

    // FN_TIMES,
    // FN_CONS,
    // FN_CAR,
    // FN_CDR,
    // FN_EQ,
    // FN_GT,
    // FN_LT,
    // FN_STRINGQ,
    // FN_STRINGEQ,
    // FN_CONSQ,
    // FN_NUMBERQ,
    // FN_FUNCTIONQ,
    // FN_USER_DEFINED_FUNCTIONQ,
    // FN_EQNR,
    // FN_FUNCTION_BODY,
    // FN_FUNCTION_ARGLIST,
    // FN_LIST,
    
    // FN_LOAD,
    // FN_OPEN_FOR_READING,
    // FN_CLOSE,
    // FN_READ,
    // FN_READ_CHAR,
    // FN_READ_LINE,
}

#[derive(Clone)]
pub struct ScmBuildInSyntax {
    pub tag: BuildInSyntax,
    pub name: String,
    pub num_args: i64,
}

#[derive(Clone)]
pub enum BuildInSyntax {
    Quote,
    Lambda,
    Define,
    If,
    Set,
    Begin,
    // SYN_QUOTE,
    // SYN_LAMBDA,
    // SYN_DEFINE,
    // SYN_IF,
    // SYN_SET,
    // SYN_BEGIN,
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
        ScmObject::CONS(Cons {
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

        ScmObject::FN(ScmBuildInFunction {
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
        ScmObject::USERFN(ScmUserFunction {
            name: name,
            arg_list: Box::from(arg_list),
            body_list: Box::from(body_list),
            home_environment: Box::from(home_environment),
        })
    }

    pub fn getNumber(&self) -> i64 {
        if let ScmObject::NUMBER(n) = self {
            return *n;
        }
        panic!("get Number of not a number");
    }

    pub fn equal(&self, scm: &ScmObject) -> bool {
        match self {
            ScmObject::FALSE => {
                if let ScmObject::FALSE = scm {
                    return true;
                }
                false
            }
            ScmObject::TRUE => {
                if let ScmObject::TRUE = scm {
                    return true;
                }
                false
            }
            ScmObject::SYMBOL(symbole) => {
                if let ScmObject::SYMBOL(s) = &scm {
                    if symbole == s {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}
