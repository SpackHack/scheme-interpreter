use super::lib;
use std::io::{stdin, Read};

pub fn read() -> Option<lib::ScmObject> {
    let mut character = [0];
    stdin().read(&mut character).unwrap();

    let mut char: char = character[0] as char;
    println!("Cahr Read: {}", character[0] as char);

    while is_whitespace(char) {
        stdin().read(&mut character).unwrap();
        char = character[0] as char;
    }

    if is_number(char) {
        return Some (read_number(char));
    }
    if char == '"' {
        return Some (read_chars());
    } 
    if char == ';' {
        skip_line();
        return None;
    }
    if char == '#' {
        return Some (read_bool());
    }

    Some (lib::ScmObject::new_error("Error in Reader".to_string()))
}

fn read_number(firstchar: char) -> lib::ScmObject {
    let mut number: i64 = 0;
    let mut is_negativ: bool = true;
    if firstchar != '-' {
        is_negativ = false;
        number = firstchar as i64 - '0' as i64;
    }

    let mut character = [0];
    stdin().read(&mut character).unwrap();
    let mut char: char = character[0] as char;

    while char >= '0' && char <= '9' {
        number = number * 10;
        number = number + char as i64 - '0' as i64;
        stdin().read(&mut character).unwrap();
        char = character[0] as char;
    }

    if is_negativ {
        number = number * -1;
    }

    return lib::ScmObject::new_number(number);
}

fn read_chars() -> lib::ScmObject {
    let mut chars = String::new();

    let mut character = [0];
    stdin().read(&mut character).unwrap();
    let mut char: char = character[0] as char;

    while char != '"' {
        chars.push(char);
        stdin().read(&mut character).unwrap();
        char = character[0] as char;
    }
    stdin().read(&mut character).unwrap();
    lib::ScmObject::new_chars(chars.to_string())
}

fn read_bool() -> lib::ScmObject {
    lib::ScmObject::new_bool(false)
}

fn skip_line () {
    stdin().read_line(& mut String::from("")).unwrap();
}

fn is_whitespace(character: char) -> bool {
    if character == ' ' {
        return true;
    }
    false
}

fn is_number(character: char) -> bool {
    if character >= '0' && character <= '9' || character == '-' {
        return true;
    }
    false
}

// fn is_charackter(character: char) -> bool {
//     if character <= 'a' && character >= 'z' || character <= 'A' && character >= 'Z' {
//         return true;
//     }
//     false
// }
