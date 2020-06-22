use super::lib::{ScmObject, ScmStream, Stream as s};
use std::io::Read;

fn get_char(mut scms: &mut ScmStream) -> Option<char> {
    let mut buf = [0];
    let result;
    let returnchar: char;

    if scms.readchar != '\0' {
        returnchar = scms.readchar;
        scms.readchar = '\0';
        return Some(returnchar);
    }

    match &mut scms.stream {
        s::FILE(f) => {
            result = f.read(&mut buf);
        }
        s::STDIN(a) => {
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

fn unread(stream: &mut ScmStream, c: char) {
    if stream.readchar != '\0' {
        eprintln!("Error unread second time");
    } else {
        stream.readchar = c;
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

    if is_number(c) || c == '-' {
        unread(&mut stream, c);
        return read_number(stream);
    }
    if c == '"' {
        return read_chars(stream);
    }
    if c == '(' {
        return read_list(stream);
    }

    unread(&mut stream, c);
    return read_symbol(stream);
}

// if number to big rust panic
fn read_number(stream: &mut ScmStream) -> ScmObject {
    let mut number: i64 = 0;
    let mut is_negativ: bool = true;

    match get_char(stream) {
        Some(c) => {
            if c != '-' {
                is_negativ = false;
                number = c as i64 - '0' as i64;
            }
        }
        None => {}
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
                    unread(stream, c);
                    break ScmObject::new_number(number);
                }
            }
            None => {
                break ScmObject::new_error(String::from("Error in read Number"));
            }
        }
    };
}

// Endlosschleife wenn string nicht beendet
fn read_chars(stream: &mut ScmStream) -> ScmObject {
    let mut chars = String::new();
    return loop {
        match get_char(stream) {
            Some(c) => match c {
                '"' => {
                    break ScmObject::new_chars(chars);
                }
                _ => {
                    chars.push(c);
                }
            },
            None => {
                break ScmObject::new_error(String::from("Error in read chars"));
            }
        }
    };
}

// Endlosschleife wenn list nicht beendet
fn read_list(stream: &mut ScmStream) -> ScmObject {
    let c: char = skip_whitespace(stream);

    if c == ')' {
        return ScmObject::new_nil();
    }
    // End of file

    unread(stream, c);

    let element: ScmObject = read(stream);
    let restlist: ScmObject = read_list(stream);

    return ScmObject::new_cons(element, restlist);
}

fn read_symbol(stream: &mut ScmStream) -> ScmObject {

    match get_char(stream) {
        Some(c) => match c {
            '#' => {
                // read tags
                return ScmObject::new_error(String::from("not implemented"));
            }
            _ => {

            }
        }
        None => {

        }
    }


    loop {
        match get_char(stream) {
            Some(c) => match c {
                ' ' => {
                    return ScmObject::new_nil();
                }
                _ => {
                
                }
            }
            None => {}
        }
    }

    // match option_c {
    //     Some(c) => match c {
    //         ';' => {}
    //         '#' => {
    //             return ScmObject::new_error(String::from("not implemented"));
    //         }
    //         _ => {
    //             return ScmObject::new_error(String::from("not implemented"));
    //         }
    //     },
    //     None => {
    //         return ScmObject::new_error(String::from("Error in read symbole"));
    //     }
    // }
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
    if character == ' ' || character == '\n' || character == '\t' {
        return true;
    }
    false
}

fn is_number(character: char) -> bool {
    if character >= '0' && character <= '9' {
        return true;
    }
    false
}
