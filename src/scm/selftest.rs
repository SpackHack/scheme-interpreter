use super::lib::{ScmObject, ObjectType};

fn assert(check: bool, message: String) {
    if !check {
        println!("Selftest Error: {}", message);
    }
} 

pub fn selftest() {
    let o = ScmObject::new_number(123);
   //assert(o.value == ObjectType::NUMBER, String::from("wrong Tag should be Number"));
}