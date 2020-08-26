use std::fs::File;
use std::io::{stdin, Stdin};

pub struct ScmStream {
    pub stream: Stream,
    pub read_char: Vec<char>,
}

pub enum Stream {
    FILE(File),
    STDIN(Stdin),
}

impl ScmStream {
    pub fn new_file(file: File) -> Self {
        ScmStream {
            stream: Stream::FILE(file),
            read_char: vec![],
        }
    }

    pub fn new_stdin() -> Self {
        ScmStream {
            stream: Stream::STDIN(stdin()),
            read_char: vec![],
        }
    }
}