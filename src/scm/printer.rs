use super::scm_object::{Cons, ScmObject};
use std::io;
use std::io::Write;

pub fn print_result(input: ScmObject) {
    match input {
        ScmObject::None => {}
        ScmObject::Void => {}
        _ => {
            io::stdout().flush().unwrap();
            print!("> ");
            print(input, true);
        }
    }
}

pub fn display_or_print(scm: ScmObject, do_print: bool) {
    match scm {
        ScmObject::None => {}
        ScmObject::Void => {}
        _ => {
            io::stdout().flush().unwrap();
            print(scm, do_print);
        }
    }
}

fn print(input: ScmObject, do_print: bool) {
    match input {
        ScmObject::ERROR(error) => {
            print!("{}", error);
        }
        ScmObject::NUMBER(numbers) => {
            print!("{}", numbers);
        }
        ScmObject::STRING(chars) => {
            if do_print {
                print!("\"{}\"", chars);
            } else {
                print!("{}", chars);
            }
        }
        ScmObject::CONS(cons) => {
            print!("(");
            print_list(cons);
        }
        ScmObject::SYMBOL(symbole) => {
            print!("{}", symbole);
        }
        ScmObject::TRUE => {
            print!("#T");
        }
        ScmObject::FALSE => {
            print!("#F");
        }
        ScmObject::NULL => {
            print!("#N");
        }
        ScmObject::USERFN(user_fn) => {
            print!("(lambda (",);
            if let ScmObject::CONS(cons) = *user_fn.arg_list {
                print_list(cons);
            }
            print!(" (");
            if let ScmObject::CONS(cons) = *user_fn.body_list {
                print_list(cons);
            }
            print!(" )");
        }
        _ => print!("Print Not implemented"),
    }
}

fn print_list(list: Cons) {
    print(*list.car, true);

    let cdr: ScmObject = *list.cdr;

    match cdr {
        ScmObject::NIL => {
            print!(".\\)");
        }
        _ => {
            print!(".");
            print(cdr, true);
            print!(")");
        }
    }
}
