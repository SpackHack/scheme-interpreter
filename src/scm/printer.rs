use super::scmObject::{Cons, ScmObject};
use std::io;
use std::io::Write;

pub fn print_result(input: ScmObject) {

    match input {
        ScmObject::None => {},
        ScmObject::Void => {},
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
            print!("(");
            print_list(cons);
            print!(")");
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
        _ => println!("Print Not implemented"),
    }
}

fn print_list(list: Cons) {
    print(*list.car);

    let cdr: ScmObject = *list.cdr;

    match cdr {
        ScmObject::NIL => {}
        _ => {
            print!(" . ");
            print(cdr);
        }
    }
}
