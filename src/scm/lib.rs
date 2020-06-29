use std::io::{Stdin, stdin};
use std::fs::File;

#[derive(Clone)]
pub struct ScmObject {
    pub value: ObjectType,
}

#[derive(Clone)]
pub enum ObjectType {
    ERROR(String),
    NUMBER(i64),
    STRING(String),
    CONS(Cons),
    NIL,
    SYMBOL(String),
    TRUE,
    FALSE,
    NULL,
    FN(ScmBuildInFunction),
    USERFN(UserFunction),
    EOF,
}

#[derive(Clone)]
pub enum BuildInFunction {
    Quote,
    FnPlus,
}

#[derive(Clone)]
pub struct ScmBuildInFunction {
    pub tag: BuildInFunction,
    pub name: String,
    pub numArgs: i64,
}

#[derive(Clone)]
pub struct UserFunction {
    pub name: String,
    pub numArgs: i64,
    pub argList: Box<ScmObject>,
    pub bodyList: Box<ScmObject>,
    pub homeEnvironment: Box<ScmObject>,
}

impl ScmObject {
    pub fn new_error(chars: String) -> Self {
        ScmObject {
            value: ObjectType::ERROR(chars),
        }
    }

    pub fn new_number(number: i64) -> Self {
        ScmObject {
            value: ObjectType::NUMBER(number),
        }
    }

    pub fn new_chars(string: String) -> Self {
        ScmObject {
            value: ObjectType::STRING(string),
        }
    }

    pub fn new_cons(new_car: ScmObject, new_cdr: ScmObject) -> Self {
        ScmObject {
            value: ObjectType::CONS(Cons {
                car: Box::new(new_car),
                cdr: Box::new(new_cdr),
            }),
        }
    }

    pub fn new_nil() -> Self {
        ScmObject {
            value: ObjectType::NIL,
        }
    }

    pub fn new_symbol(symbole: String) -> Self {
        ScmObject {
            value: ObjectType::SYMBOL(symbole),
        }
    }

    pub fn new_true() -> Self {
        ScmObject {
            value: ObjectType::TRUE,
        }
    }

    pub fn new_false() -> Self {
        ScmObject {
            value: ObjectType::FALSE,
        }
    }

    pub fn new_null() -> Self {
        ScmObject {
            value: ObjectType::NULL,
        }
    }

    pub fn new_eof() -> Self {
        ScmObject {
            value: ObjectType::EOF,
        }
    }
}

#[derive(Clone)]
pub struct Cons {
    pub car: Box<ScmObject>,
    pub cdr: Box<ScmObject>,
}

pub struct ScmStream {
    pub stream: Stream,
    pub readchar: char,
}

pub enum Stream {
    FILE(File),
    STDIN(Stdin),
}

impl ScmStream {
    pub fn new_file(file: File) -> Self {
        ScmStream {
            stream: Stream::FILE(file),
            readchar: '\0',
        }
    }

    pub fn new_stdin() -> Self {
        ScmStream {
            stream: Stream::STDIN(stdin()),
            readchar: '\0',
        }
    }
}
