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
        ScmObject::Error(error) => {
            print!("{}", error);
        }
        ScmObject::Number(numbers) => {
            print!("{}", numbers);
        }
        ScmObject::Chars(chars) => {
            if do_print {
                print!("\"{}\"", chars);
            } else {
                print!("{}", chars);
            }
        }
        ScmObject::Cons(cons) => {
            print!("(");
            print_list(cons);
        }
        ScmObject::Symbol(symbole) => {
            print!("{}", symbole);
        }
        ScmObject::Function(function) => {

        }
        ScmObject::Syntax(syntax) => {
            
        }
        ScmObject::UserFunction(user_fn) => {
            print!("(lambda (",);
            if let ScmObject::Cons(cons) = *user_fn.arg_list {
                print_list(cons);
            }
            print!(" (");
            if let ScmObject::Cons(cons) = *user_fn.body_list {
                print_list(cons);
            }
            print!(" )");
        }
        ScmObject::EndOfFile => {
            print!("#EOF");
        }
        ScmObject::Null => {
            print!("#N");
        }
        ScmObject::Void => {
            print!("#V");
        }
        ScmObject::True => {
            print!("#T");
        }
        ScmObject::False => {
            print!("#F");
        }
        _ => print!("Print Not implemented"),
    }
}

fn print_list(list: Cons) {
    print(*list.car, true);

    let cdr: ScmObject = *list.cdr;

    match cdr {
        ScmObject::Nil => {
            print!(".\\)");
        }
        _ => {
            print!(".");
            print(cdr, true);
            print!(")");
        }
    }
}
