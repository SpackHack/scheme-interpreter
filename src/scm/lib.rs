pub struct ScmObject {
    pub value: Value,
}

pub enum Value {
    Error(String),
    Number(i64),
    Chars(String),
    Bool(bool),
    None,
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

    pub fn new_bool(bool_value: bool) -> Self {
        ScmObject {
            value: Value::Bool(bool_value),
        }
    }

    pub fn new_null() -> Self {
        ScmObject {
            value: Value::None,
        }
    }
}
