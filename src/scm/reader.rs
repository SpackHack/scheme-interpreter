use super::lib::{ScmObject};
use std::io::{stdin, Read};

fn next_char() -> Option<char> {
    let mut character = [0];
    match stdin().read(&mut character) {
        OK => {
            return Some (character[0] as char);
        }
        err => {
            return None;
        }
    }
    
}

pub fn read() -> Option<ScmObject> {
    return loop {
        match next_char() {
            Some (c) => {
    
                if is_whitespace(c) {
                    continue;
                    // skip_whitespace();
                }
                if is_number(c) {
                    break Some (read_number(c));
                }
                if c == '"' {
                    break Some (read_chars());
                }
                if c == ';' {
                    skip_line();
                    break None;
                }
                if c == '#' {
                    break Some (read_const());
                }
                if c == '\n' {
                    break None;
                }
                println!("ERR {} : {}", c , c as i64);
                break Some (ScmObject::new_error("Error in Reader".to_string()));
            }
            None => {
                break Some (ScmObject::new_error("Error in next Cahr".to_string()))
            }
        }
    }
}

fn read_number(firstchar: char) -> ScmObject {
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

    return ScmObject::new_number(number);
}

// Endlosschleife wenn string nicht beendet
fn read_chars() -> ScmObject {
    let mut chars = String::new();
    return loop {
        match next_char() {
            Some (c) => {
                match c {
                    '"' => {
                        break ScmObject::new_chars(chars);  
                    }
                    _ => {
                        chars.push(c);
                    }
                }
            }
            None => {
                break ScmObject::new_error(String::from("Error in read chars"));
            }
        }
    }
}

fn read_const() -> ScmObject {
    match next_char() {
        Some (character) => {
            match character {
                'T' => {
                    return ScmObject::new_bool(true);
                }
                'F' => {
                    return ScmObject::new_bool(false);
                }
                'N' => {
                    return ScmObject::new_null();
                }
                _ => {
                    let mut err: String = String::from("Const is not implementet: ");
                    err.push(character);
                    return ScmObject::new_error(err);
                }
            }
        }
        None => {
            return ScmObject::new_error(String::from("Error in Reader"));
        }
    }
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
