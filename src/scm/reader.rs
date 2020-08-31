use super::memory::new_symbole;
use super::scm_object::{ScmObject};
use super::stream::*;
use std::io::Read;

fn get_char(scm_stream: &mut ScmStream) -> Option<char> {
    let mut buf = [0];
    let result;

    if !scm_stream.read_char.is_empty() {
        return scm_stream.read_char.pop();
    }

    match &mut scm_stream.stream {
        Stream::FILE(f) => {
            result = f.read(&mut buf);
        }
        Stream::STDIN(a) => {
            result = a.read(&mut buf);
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

fn unread_vector(stream: &mut ScmStream, vec: Vec<char>) {
    for elem in vec {
        unread_char(stream, elem);
    }
}

pub fn read(mut stream: &mut ScmStream) -> ScmObject {
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

    if is_type_number(c, stream) {
        return read_number(c, stream);
    } else if c == '"' {
        return read_chars(stream);
    } else if c == '(' {
        return read_list(stream);
    } else if is_end_of_file(c) {
        return ScmObject::EndOfFile;
    }

    unread_char(&mut stream, c);
    return read_symbol(stream);
}

// if number to big rust panic
fn read_number(c: char, stream: &mut ScmStream) -> ScmObject {
    let mut number: i64 = 0;
    let mut is_negativ: bool = true;

    if c != '-' {
        is_negativ = false;
        number = c as i64 - '0' as i64;
    }

    return loop {
        match get_char(stream) {
            Some(c) => {
                if is_number(c) {
                    number = number * 10;
                    number = number + c as i64 - '0' as i64;
                } else {
                    if is_negativ {
                        number = number * -1;
                    }
                    unread_char(stream, c);
                    break ScmObject::Number(number);
                }
            }
            None => {
                break ScmObject::Error(String::from("Error in read Number"));
            }
        }
    };
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

// Endlosschleife wenn list nicht beendet
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
                // read tags
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
                    //return ScmObject::new_symbol(symbole);
                }
                ';' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                    //return ScmObject::new_symbol(symbole);
                }
                '\n' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                    //return ScmObject::new_symbol(symbole);
                }
                ')' => {
                    unread_char(stream, c);
                    return unsafe { new_symbole(symbole) };
                    //return ScmObject::new_symbol(symbole);
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
                if !is_whitespace(c) {
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

fn is_whitespace(character: char) -> bool {
    if character == ' ' || character == '\n' || character == '\t' || character == '\r' {
        return true;
    }
    false
}

fn is_end_of_file(character: char) -> bool {
    if character as i64 == 0 {
        return true;
    }
    false
}

fn is_type_number(mut character: char, scm_stream: &mut ScmStream) -> bool {
    let mut chars: Vec<char> = vec![];
    if character == '-' {
        character = get_char(scm_stream).unwrap();
        chars.push(character);
        if is_whitespace(character) || is_end_of_file(character) {
            chars.reverse();
            unread_vector(scm_stream, chars);
            return false;
        }
    } else {
        if !is_number(character) {
            return false;
        }
    }

    character = get_char(scm_stream).unwrap();
    let mut re: bool = true;

    while !is_end_of_type(character) {
        if !is_number(character) {
            re = false;
            break;
        }

        chars.push(character);
        character = get_char(scm_stream).unwrap();
    }
    chars.push(character);
    chars.reverse();
    unread_vector(scm_stream, chars);
    return re;
}

fn is_number(character: char) -> bool {
    if character >= '0' && character <= '9' {
        return true;
    }
    false
}

fn is_end_of_type(c: char) -> bool {
    if is_whitespace(c) || is_end_of_file(c) || c == ';' || c == ')' {
        return true;
    }
    false
}
