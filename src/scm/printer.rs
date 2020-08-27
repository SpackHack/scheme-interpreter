use super::scmObject::{Cons, ScmObject};
use std::io;
use std::io::Write;

pub fn print_result(input: ScmObject) {
    match input {
        ScmObject::None => {}
        ScmObject::Void => {}
        _ => {
            io::stdout().flush().unwrap();
            print!("> ");
            print(input);
            println!();
        }
    }
}

fn print(input: ScmObject) {
    match input {
        ScmObject::ERROR(error) => {
            print!("{}", error);
        }
        ScmObject::NUMBER(numbers) => {
            print!("{}", numbers);
        }
        ScmObject::STRING(chars) => {
            print!("{}", chars);
        }
        ScmObject::CONS(cons) => {
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
            println!(
                "Funktion: {}",
                user_fn.name.unwrap_or(String::from("NO_NAME"))
            );
            print!("Args: ");
            print(*user_fn.arg_list);
            println!();
            print!("Body: ");
            print(*user_fn.body_list);
            println!();
            println!("Env: ");
        }
        _ => println!("Print Not implemented"),
    }
}

fn print_list(list: Cons) {
    print!("(");
    print(*list.car);

    let cdr: ScmObject = *list.cdr;

    match cdr {
        ScmObject::NIL => {
            print!(")");
        }
        _ => {
            print!(" .");
            print(cdr);
            print!(")");
        }
    }
}
