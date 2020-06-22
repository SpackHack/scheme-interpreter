use std::io::{Stdin, stdin};
use std::fs::File;


pub struct ScmObject {
    pub value: Value,
}

pub enum Value {
    ERROR(String),
    NUMBER(i64),
    STRING(String),
    CONS(Cons),
    NIL,
    SYMBOL(String),
    TRUE,
    FALSE,
    NULL,
}

impl ScmObject {
    pub fn new_error(chars: String) -> Self {
        ScmObject {
            value: Value::ERROR(chars),
        }
    }

    pub fn new_number(number: i64) -> Self {
        ScmObject {
            value: Value::NUMBER(number),
        }
    }

    pub fn new_chars(string: String) -> Self {
        ScmObject {
            value: Value::STRING(string),
        }
    }

    pub fn new_cons(new_car: ScmObject, new_cdr: ScmObject) -> Self {
        ScmObject {
            value: Value::CONS(Cons {
                car: Box::new(new_car),
                cdr: Box::new(new_cdr),
            }),
        }
    }

    pub fn new_nil() -> Self {
        ScmObject {
            value: Value::NIL,
        }
    }

    pub fn new_symbol(symbole: String) -> Self {
        ScmObject {
            value: Value::SYMBOL(symbole),
        }
    }

    pub fn new_true() -> Self {
        ScmObject {
            value: Value::TRUE,
        }
    }

    pub fn new_false() -> Self {
        ScmObject {
            value: Value::FALSE,
        }
    }

    pub fn new_null() -> Self {
        ScmObject {
            value: Value::NULL,
        }
    }
}

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
