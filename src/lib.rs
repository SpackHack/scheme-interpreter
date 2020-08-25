use std::fs::File;
use std::io::{stdin, Stdin};

#[derive(Clone)]
pub enum ScmObject {
    ERROR(String),
    NUMBER(i64),
    STRING(String),
    CONS(Cons),
    NIL,            // end of list
    SYMBOL(String),
    TRUE,
    FALSE,
    NULL,
    Void,
    FN(ScmBuildInFunction),
    Syntax(ScmBuildInSyntax),
    USERFN(UserFunction),
    EOF,
    ENV(ScmEnvironment),
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
    pub numArgs: i64,
}

#[derive(Clone)]
pub enum BuildInFunction {
    Plus,
    Minus,
}

#[derive(Clone)]
pub struct ScmBuildInSyntax{
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
    pub numArgs: i64,
    pub argList: Box<ScmObject>,
    pub bodyList: Box<ScmObject>,
    pub homeEnvironment: Box<ScmObject>,
}

#[derive(Clone)]
pub struct ScmEnvironment {
    pub parent_env: Box<ScmObject>,
    pub bindings: Vec<ScmObject>,
    pub num_bindigs: i64,
}

pub struct ScmStream {
    pub stream: Stream,
    pub readchar: Vec<char>,
}

pub enum Stream {
    FILE(File),
    STDIN(Stdin),
}

impl ScmObject {
    pub fn new_error(chars: String) -> Self {
        ScmObject::ERROR(chars)
    }

    pub fn new_number(number: i64) -> Self {
        ScmObject::NUMBER(number)
    }

    pub fn new_chars(string: String) -> Self {
        ScmObject::STRING(string)
    }

    pub fn new_cons(new_car: ScmObject, new_cdr: ScmObject) -> Self {
        ScmObject::CONS(Cons {
            car: Box::new(new_car),
            cdr: Box::new(new_cdr),
        })
    }

    pub fn new_nil() -> Self {
        ScmObject::NIL
    }

    pub fn new_symbol(symbole: String) -> Self {
        ScmObject::SYMBOL(symbole)
    }

    pub fn new_true() -> Self {
        ScmObject::TRUE
    }

    pub fn new_false() -> Self {
        ScmObject::FALSE
    }

    pub fn new_null() -> Self {
        ScmObject::NULL
    }

    pub fn new_eof() -> Self {
        ScmObject::EOF
    }

    pub fn new_fn(tag: BuildInFunction, name: String, num_of_args: i64) -> Self {
        ScmObject::FN(ScmBuildInFunction {
            tag: tag,
            name: name,
            numArgs: num_of_args,
        })
    }

    pub fn new_syntax(tag: BuildInSyntax, name: String, num_of_args: i64) -> Self {
        ScmObject::Syntax(ScmBuildInSyntax {
            tag: tag,
            name: name,
            num_args: num_of_args,
        })
    }

    pub fn new_env() -> Self {
        ScmObject::ENV(ScmEnvironment {
            parent_env: Box::new(ScmObject::new_null()),
            bindings: vec![],
            num_bindigs: 0,
        })
    }
}

impl ScmStream {
    pub fn new_file(file: File) -> Self {
        ScmStream {
            stream: Stream::FILE(file),
            readchar: vec![],
        }
    }

    pub fn new_stdin() -> Self {
        ScmStream {
            stream: Stream::STDIN(stdin()),
            readchar: vec![],
        }
    }
}

pub fn scm_equal(scm1: &ScmObject, scm2: &ScmObject) -> bool {
    return match &scm1 {
        ScmObject::SYMBOL(symbole) => {
            if let ScmObject::SYMBOL(s) = &scm2 {
                if symbole == s {
                    return true;
                }
            }
            false
        }
        _ => false,
    };
}
