use super::scm_object::ScmObject;

fn assert(check: bool, message: String) {
    if !check {
        println!("Selftest Error: {}", message);
    }
}

pub fn selftest() {
    let o = ScmObject::Number(123);
    assert(
        matches!(o,  ScmObject::Number(number)),
        String::from("wrong Tag should be Number"),
    );
}
