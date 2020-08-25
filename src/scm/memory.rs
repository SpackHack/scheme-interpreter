use scheme_lib::{ScmObject};


static mut NUMBER_OF_SYMBOLES: i64 = 0;
static mut SYMBOLES: Vec<ScmObject> = Vec::new();

static mut SCM_TRUE: Option<ScmObject> = None;
static mut SCM_FALSE: Option<ScmObject> = None;
static mut SCM_NULL: Option<ScmObject> = None;

pub unsafe fn init_singeltons() {
    SCM_TRUE = Some(ScmObject::new_true());
    SCM_FALSE = Some(ScmObject::new_false());
    SCM_NULL = Some(ScmObject::new_null());
}

pub unsafe fn new_symbole(symbole: String) -> ScmObject {

    let a = get_existing_symbole(&symbole);

    if let None = a {
        return add_symbole(symbole);
    } else {
        return a.unwrap();
    }
}

unsafe fn add_symbole<'a>(symbole: String) -> ScmObject {

    let scm = ScmObject::new_symbol(symbole);

    SYMBOLES.push(scm);
    NUMBER_OF_SYMBOLES = NUMBER_OF_SYMBOLES + 1;

    return SYMBOLES.last().unwrap().clone();
}

unsafe fn get_existing_symbole<'a>(symbole: &String) -> Option<ScmObject> {
    for i in SYMBOLES.iter() {
        match &i {
            ScmObject::SYMBOL(s) => {
                if *s == *symbole {
                    return Some(i.clone());
                }
            }
            _ => {

            }
        }
    }
    None
}