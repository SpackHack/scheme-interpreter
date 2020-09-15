use std::fs::File;
use std::io::Stdin;
use std::rc::Rc;

#[derive(Clone)]
pub struct ScmStream {
    pub stream_type: StreamType,
    pub read_char: Vec<char>,
}

#[derive(Clone)]
pub enum StreamType {
    FILE(Rc<File>),
    STDIN(Rc<Stdin>),
}
