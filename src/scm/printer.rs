use super::lib::{ScmObject, Value as Type};

pub fn print(input: ScmObject) {
    print!(">");
    match input.value {
        Type::Error(error) => {
            println!("{}", error);
        }
        Type::Number(numbers) => {
            println!("{}", numbers);
        }
        Type::Chars(chars) => {
            println!("{}", chars);
        }
        Type::Bool(bool) => {
            println!("{}", bool);
        }
        Type::None => {
            println!("NULL");
        }
        _ => println!("Not implementet"),
    }
}
