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
        Type::Error(error) => {
            print!("{}", error);
        }
        Type::Number(numbers) => {
            print!("{}", numbers);
        }
        Type::Chars(chars) => {
            print!("{}", chars);
        }
        Type::Cons(cons) => {
            print_list(cons);
        }
        _ => println!("Not implementet"),
    }
}

fn print_list(list: Cons) {
    print(*list.car);

    let cdr: ScmObject = *list.cdr;

    match cdr.value {
        Type::Nil => {

        }
        _ => {
            print!(" ");
            print(cdr);
        }
    }
}
