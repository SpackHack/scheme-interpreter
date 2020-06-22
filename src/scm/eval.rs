use super::lib::{ScmObject, Value};

pub fn eval(mut input: &mut ScmObject) {

    match &input.value {
        Value::ERROR(error) => {}
        Value::NUMBER(number) => {}
        Value::STRING(string) => {}
        Value::CONS(cons) => {}
        Value::NIL => {}
        Value::SYMBOL(symbole) => {}
        Value::TRUE => {}
        Value::FALSE => {}
        Value::NULL => {}
        _ => {}
    }
}