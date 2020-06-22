use std::io::{Stdin, stdin};
use std::fs::File;


pub struct ScmObject {
    pub value: Value,
}

pub enum Value {
    Error(String),
    Number(i64),
    Chars(String),
    Cons(Cons),
    Nil,
    SYMBOL(String),
}

impl ScmObject {
    pub fn new_error(chars: String) -> Self {
        ScmObject {
            value: Value::Error(chars),
        }
    }

    pub fn new_number(number: i64) -> Self {
        ScmObject {
            value: Value::Number(number),
        }
    }

    pub fn new_chars(string: String) -> Self {
        ScmObject {
            value: Value::Chars(string),
        }
    }

    pub fn new_cons(new_car: ScmObject, new_cdr: ScmObject) -> Self {
        ScmObject {
            value: Value::Cons(Cons {
                car: Box::new(new_car),
                cdr: Box::new(new_cdr),
            }),
        }
    }

    pub fn new_nil() -> Self {
        ScmObject {
            value: Value::Nil,
        }
    }

    pub fn new_symbol(symbole: String) -> Self {
        ScmObject {
            value: Value::SYMBOL(symbole),
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

    // TODO:
    pub fn new_stdin() -> Self {
        ScmStream {
            stream: Stream::STDIN(stdin()),
            readchar: '\0',
        }
    }
}
