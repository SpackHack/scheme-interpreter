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
    USERFN(UserFunction),
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
    pub num_args: i64,
}

#[derive(Clone)]
pub enum BuildInFunction {
    Plus,
    Minus,
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
}

#[derive(Clone)]
pub struct UserFunction {
    pub name: String,
    pub num_args: i64,
    pub arg_list: Box<ScmObject>,
    pub body_list: Box<ScmObject>,
    pub home_environment: Box<ScmObject>,
}

impl ScmObject {

    pub fn new_cons(car: ScmObject, cdr: ScmObject) -> Self {
        ScmObject::CONS(Cons{
            car: Box::new(car),
            cdr: Box::new(cdr),
        })
    }

    pub fn new_fn(tag: BuildInFunction, name: String, num_of_args: i64) -> Self {
        ScmObject::FN(ScmBuildInFunction {
            tag: tag,
            name: name,
            num_args: num_of_args,
        })
    }

    pub fn new_syntax(tag: BuildInSyntax, name: String, num_of_args: i64) -> Self {
        ScmObject::Syntax(ScmBuildInSyntax {
            tag: tag,
            name: name,
            num_args: num_of_args,
        })
    }

    pub fn equal(&self, scm: &ScmObject) -> bool {
        match self {
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