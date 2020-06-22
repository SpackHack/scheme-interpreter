use super::lib::{Cons, ScmObject, Value as Type};
use std::io;
use std::io::Write;

pub fn print_result(input: ScmObject) {
    io::stdout().flush().unwrap();
    print!("> ");
    print(input);
    println!();
}

fn print(input: ScmObject) {
    match input.value {
        Type::ERROR(error) => {
            print!("{}", error);
        }
        Type::NUMBER(numbers) => {
            print!("{}", numbers);
        }
        Type::STRING(chars) => {
            print!("{}", chars);
        }
        Type::CONS(cons) => {
            print!("(");
            print_list(cons);
            print!(")");
        }
        Type::SYMBOL(symbole) => {
            print!("{}", symbole);
        }
        Type::TRUE => {
            print!("#T");
        }
        Type::FALSE => {
            print!("#F");
        }
        Type::NULL => {
            print!("#N");
        }
        _ => println!("Not implementet"),
    }
}

fn print_list(list: Cons) {
    print(*list.car);

    let cdr: ScmObject = *list.cdr;

    match cdr.value {
        Type::NIL => {

        }
        _ => {
            print!(" . ");
            print(cdr);
        }
    }
}
