// use super::environment::*;
// use scheme_lib::*;

// pub fn eval(input: ScmObject, mut env: &mut ScmEnvironment) -> ScmObject {
//     let a = input.clone();
//     match input {
//         ScmObject::CONS(cons) => {
//             let func = *cons.car;
//             let eval_func = eval(func, env);

//             match eval_func {
//                 // ScmObject::FN(function) => build_in_functions(function, *cons.cdr, env),
//                 // ScmObject::Syntax(syntax) => build_in_syntax(syntax, *cons.cdr, &mut env),
//                 // ScmObject::USERFN(function) => ScmObject::new_error(String::from("User FN ")),
//                 _ => ScmObject::new_error(String::from("not a func")),
//             }
//         }
//         ScmObject::SYMBOL(symbole) => {
//             let result = get_environment(&env, a);
//             if let ScmObject::None = result {
//                 return ScmObject::ERROR(String::from("Symbole not found"));
//             } else {
//                 return result;
//             }
//         }
//         _ => input,
//     }
// }