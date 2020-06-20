use super::lib::{Cons, ScmObject, Value as Type};

pub fn print_result(input: ScmObject) {
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
        Type::Bool(bool) => {
            print!("{}", bool);
        }
        Type::None => {
            print!("NULL");
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
            println!();
        }
        _ => {
            print(cdr);
        }
    }
}
