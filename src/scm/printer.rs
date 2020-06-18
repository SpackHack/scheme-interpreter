use super::lib;

pub fn print(input: lib::ScmObject) {
    print!(">");
    match input.value {
        lib::Value::Error (error) => {
            println!("{}", error);
        }
        lib::Value::Number (numbers) => {
            println!("{}", numbers);
        }
        lib::Value::Chars (chars) => {
            println!("{}", chars);
        }
        _ => {
            println!("Not implementet")
        }
    }
}
