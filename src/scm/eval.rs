use super::lib::{ObjectType, ScmObject};

pub fn eval(input: ScmObject) -> ScmObject {
    match input.value {
        ObjectType::ERROR(error) => {}
        ObjectType::NUMBER(number) => {}
        ObjectType::STRING(string) => {}
        ObjectType::CONS(cons) => {
            match cons.car.value {
                ObjectType::SYMBOL(symbole) => {
                    match symbole.as_str() {
                        "'" => {
                            return quote(*cons.cdr);
                            //return *cons.cdr.value;
                        }
                        _ => {}
                    }
                }
                _ => {

                }
            }
        }
        ObjectType::NIL => {}
        ObjectType::SYMBOL(symbole) => {}
        ObjectType::TRUE => {}
        ObjectType::FALSE => {}
        ObjectType::NULL => {}
        _ => {}
    }

    return ScmObject::new_error(String::from("no eval"));
}

fn quote(restlist: ScmObject) -> ScmObject {
    if let ObjectType::CONS(cons) = restlist.value {
        if let ObjectType::NIL = cons.cdr.value {
            return *cons.car;
        } else {
            return ScmObject::new_error(String::from("Quote restlist has more than one element"));
        }
    } else {
        return ScmObject::new_error(String::from("Quote restlist is no list"));
    }
}