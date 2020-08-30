use super::scm_object::ScmObject;

static mut NUMBER_OF_SYMBOLES: i64 = 0;
static mut SYMBOLES: Vec<ScmObject> = Vec::new();

pub unsafe fn new_symbole(symbole: String) -> ScmObject {
    let a = get_existing_symbole(&symbole);

    if let None = a {
        return add_symbole(symbole);
    } else {
        return a.unwrap();
    }
}

unsafe fn add_symbole<'a>(symbole: String) -> ScmObject {
    let scm = ScmObject::Symbol(symbole);

    SYMBOLES.push(scm);
    NUMBER_OF_SYMBOLES = NUMBER_OF_SYMBOLES + 1;

    return SYMBOLES.last().unwrap().clone();
}

unsafe fn get_existing_symbole<'a>(symbole: &String) -> Option<ScmObject> {
    for i in SYMBOLES.iter() {
        match &i {
            ScmObject::Symbol(s) => {
                if *s == *symbole {
                    return Some(i.clone());
                }
            }
            _ => {}
        }
    }
    None
}
