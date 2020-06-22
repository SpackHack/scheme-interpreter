use super::lib::{ScmObject};

static NUMBER_OF_SYMBOLES: &'static i64 = &0;
static SYMBOLES: &'static [ScmObject] = &[];

pub fn new_symbole(symbole: String) -> ScmObject{

    for i in SYMBOLES.iter() {
        
    }


    ScmObject::new_symbol(symbole)
}