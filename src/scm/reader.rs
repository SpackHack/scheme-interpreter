use super::memory::new_symbole;
use super::scm_object::{NumberType, ScmObject};
use super::stream::*;
use std::io::Read;
use std::rc::Rc;

fn get_char(scm_stream: &mut ScmStream) -> Option<char> {
    let mut buf = [0];
    let result;

    if !scm_stream.read_char.is_empty() {
        return scm_stream.read_char.pop();
    }

    match &mut scm_stream.stream_type {
        StreamType::FILE(f) => {
            let file;
            unsafe {
                file = Rc::get_mut_unchecked(f);
            }

            result = file.read(&mut buf);
        }
        StreamType::STDIN(s) => {
            let input;
            unsafe {
                input = Rc::get_mut_unchecked(s);
            }
            result = input.read(&mut buf);
        }
        StreamType::None => {
            println!("Read Error no Stream");
            return None;
        }
    }

    match result {
        Ok(_o) => {
            return Some(buf[0] as char);
        }
        Err(e) => {
            eprintln!("Read Error: {}", e);
            return None;
        }
    }
}

fn unread_char(stream: &mut ScmStream, c: char) {
    stream.read_char.push(c);
}

fn unread_string(stream: &mut ScmStream, vec: String) {
    for (_, c) in vec.char_indices() {
        unread_char(stream, c);
    }
}

pub fn scm_read(scm_stream: &mut ScmObject) -> ScmObject {
    if let ScmObject::Stream(s) = scm_stream {
        return read(s);
    }
    return ScmObject::Error(String::from("Read error Scm Stream is not a Stream"));
}

fn read(mut stream: &mut ScmStream) -> ScmObject {
    let mut again: bool = true;
    let mut c: char = skip_whitespace(stream);

    while again {
        if c == ';' {
            skip_line(stream);
            c = skip_whitespace(stream);
        } else {
            again = false;
        }
    }

    if is_number(c) || c == '-' {
        if let Some(number_type) = is_type_number(c, stream) {
            match number_type {
                NumberType::Integer(chars) => {
                    return read_integer(chars);
                }
                NumberType::Float(chars) => {
                    return read_float(chars);
                }
            };
        }
    } else if c == '"' {
        return read_chars(stream);
    } else if c == '(' {
        return read_list(stream);
    } else if c == '\'' {
        let a = read(stream);
        let cons: ScmObject;
        match a {
            ScmObject::Nil => return ScmObject::Null,
            _ => cons = ScmObject::new_cons(a, ScmObject::Nil),
        }
        return ScmObject::new_cons(ScmObject::Symbol(String::from("quote")), cons);
    } else if is_end_of_file(&c) {
        return ScmObject::EndOfFile;
    }

    unread_char(&mut stream, c);
    return read_symbol(stream);
}

fn read_integer(chars: String) -> ScmObject {
    match chars.parse::<i64>() {
        Ok(n) => ScmObject::Integer(n),
        Err(_e) => ScmObject::Error(String::from("Not a Integer Number")),
    }
}

fn read_float(chars: String) -> ScmObject {
    match chars.parse::<f64>() {
        Ok(n) => ScmObject::Float(n),
        Err(_e) => ScmObject::Error(String::from("Not a Float Number")),
    }
}

fn read_chars(stream: &mut ScmStream) -> ScmObject {
    let mut chars = String::new();
    return loop {
        match get_char(stream) {
            Some(c) => match c {
                '"' => {
                    break ScmObject::Chars(chars);
                }
                _ => {
                    chars.push(c);
                }
            },
            None => {
                break ScmObject::Error(String::from("Error in read chars"));
            }
        }
    };
}

fn read_list(stream: &mut ScmStream) -> ScmObject {
    let c: char = skip_whitespace(stream);

    if c == ')' {
        return ScmObject::Nil;
    }
    // End of file

    unread_char(stream, c);

    let element: ScmObject = read(stream);
    let restlist: ScmObject = read_list(stream);

    return ScmObject::new_cons(element, restlist);
}

fn read_symbol(stream: &mut ScmStream) -> ScmObject {
    let mut symbole: String = String::from("");

    match get_char(stream) {
        Some(c) => match c {
            '#' => {
                return read_hash(stream);
            }
            _ => {
                symbole.push(c);
            }
        },
        None => {}
    }
    loop {
        match get_char(stream) {
            Some(c) => match c {
                ' ' => {
                    // end
                    return unsafe { new_symbole(symbole) };
                }
                ';' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                }
                '\n' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                }
                ')' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                }
                _ => {
                    symbole.push(c);
                }
            },
            None => {}
        }
    }
}

fn read_hash(stream: &mut ScmStream) -> ScmObject {
    match get_char(stream) {
        Some(c) => match c {
            'T' | 't' => {
                // end
                return ScmObject::True;
            }
            'F' | 'f' => {
                // end
                return ScmObject::False;
            }
            'N' | 'n' => {
                // end
                return ScmObject::Null;
            }
            'V' | 'v' => {
                // end
                return ScmObject::Void;
            }
            _ => return ScmObject::Error(String::from("Error in hash")),
        },
        None => {
            return ScmObject::Error(String::from("Error in read has"));
        }
    }
}

fn skip_whitespace(stream: &mut ScmStream) -> char {
    loop {
        match get_char(stream) {
            Some(c) => {
                if !is_whitespace(&c) {
                    break c;
                }
            }
            None => {}
        }
    }
}

fn skip_line(stream: &mut ScmStream) -> char {
    return loop {
        match get_char(stream) {
            Some(c) => {
                if c == '\n' {
                    break c;
                }
            }
            None => {}
        }
    };
}

fn is_whitespace(character: &char) -> bool {
    if *character == ' ' || *character == '\n' || *character == '\t' || *character == '\r' {
        return true;
    }
    false
}

fn is_end_of_file(character: &char) -> bool {
    if *character as i64 == 0 {
        return true;
    }
    false
}

fn is_type_number(mut character: char, scm_stream: &mut ScmStream) -> Option<NumberType> {
    let mut chars: String = String::from("");
    let mut is_integer: bool = true;

    while !is_end_of_type(&character) {
        chars.push(character);
        character = get_char(scm_stream).unwrap();
    }

    for (index, c) in chars.char_indices() {
        match c {
            '-' => {
                if index != 0 {
                    chars.push(character);
                    chars.remove(0);
                    unread_string(scm_stream, chars);
                    return None;
                } else {
                    if let None = chars.chars().nth(index + 1) {
                        chars.push(character);
                        chars.remove(0);
                        unread_string(scm_stream, chars);
                        return None;
                    }
                }
            }
            '.' => {
                if !is_integer || index == 0 {
                    chars.push(character);
                    chars.remove(0);
                    unread_string(scm_stream, chars);
                    return None;
                }
                is_integer = false;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
            _ => {
                chars.push(character);
                chars.remove(0);
                unread_string(scm_stream, chars);
                return None;
            }
        }
    }

    unread_char(scm_stream, character);

    if is_integer {
        return Some(NumberType::Integer(chars));
    } else {
        return Some(NumberType::Float(chars));
    }
}

fn is_number(character: char) -> bool {
    if character >= '0' && character <= '9' {
        return true;
    }
    false
}

fn is_end_of_type(c: &char) -> bool {
    if is_whitespace(c) || is_end_of_file(c) || *c == ';' || *c == ')' {
        return true;
    }
    false
}
