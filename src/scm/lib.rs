pub struct ScmObject {
    //pub scm_type: ScmType,
    pub value: Value,
}

pub enum Value {
    Error(String),
    Number(i64),
    Chars(String),
    Bool(bool),
}

// pub enum ScmType {
//     ERROR = 0,
//     NUMBER = 1,
//     STRING = 2,
// }

impl ScmObject {
    pub fn new_error(chars: String) -> Self {
        ScmObject {
            // scm_type: ScmType::ERROR,
            value: Value::Error(chars.to_string()),
        }
    }

    pub fn new_number(number: i64) -> Self {
        ScmObject {
            // scm_type: ScmType::NUMBER,
            value: Value::Number(number),
        }
    }

    pub fn new_chars(chars: String) -> Self {
        ScmObject {
            // scm_type: ScmType::STRING,
            value: Value::Chars(chars.to_string()),
        }
    }

    pub fn new_bool(bool_value: bool) -> Self {
        ScmObject {
            value: Value::Bool(bool_value),
        }
    }
}
