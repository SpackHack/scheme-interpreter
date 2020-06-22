// use super::lib::{ScmObject, Value};

// static mut NUMBER_OF_SYMBOLES: &'static i64 = &0;
// static mut SYMBOLES: Option<[ScmObject]> = None;

// static mut SCM_TRUE: Option<ScmObject> = None;
// static mut SCM_FALSE: Option<ScmObject> = None;
// static mut SCM_NULL: Option<ScmObject> = None;

// pub unsafe fn init_singeltons() {
//     SCM_TRUE = Some(ScmObject::new_true());
//     SCM_FALSE = Some(ScmObject::new_false());
//     SCM_NULL = Some(ScmObject::new_null());
// }


// pub unsafe fn new_symbole(symbole: String) -> ScmObject {

//     for i in SYMBOLES.iter() {
//         if let Value::SYMBOL(s) = &i.value {
//             // if s == symbole {
//             //     return i;
//             // }
//         } 
//     }

//     ScmObject::new_symbol(symbole)
// }
