use super::lib::{ScmObject};
use std::io::{stdin, Read};

fn next_char() -> Option<char> {
    let mut character = [0];
    match stdin().read(&mut character) {
        Ok (n) => {
            return Some (character[0] as char);
        }
        Err (e) => {
            println!("Read Error: {}", e);
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
                }
                if is_number(c) || c == '-'{
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

// cut char after number
// if number to big rust panic
fn read_number(firstchar: char) -> ScmObject {
    let mut number: i64 = 0;
    let mut is_negativ: bool = true;
    if firstchar != '-' {
        is_negativ = false;
        number = firstchar as i64 - '0' as i64;
    }
    return loop {
        match next_char() {
            Some (c) => {
                if is_number(c) {
                    number = number * 10;
                    number = number + c as i64 - '0' as i64;
                } else {
                    if is_negativ {
                        number = number * -1;
                    }
                    break ScmObject::new_number(number);
                }
            }
            None => {
                break ScmObject::new_error(String::from("Error in read Number"));
            }
        }
    }
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

// cut char after const
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
    if character >= '0' && character <= '9'{
        return true;
    }
    false
}
