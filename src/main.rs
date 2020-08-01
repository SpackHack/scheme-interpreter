mod scm;
use scheme_lib::{*};
use std::env;
use std::fs::File;

fn main() {

    scm::selftest::selftest();
    let mut init: bool = true;
    // unsafe {scm::memory::init_singeltons()};

    // unsafe {scm::memory::new_symbole(String::from("symbole: String"))};
    let args: Vec<String> = env::args().collect();

    let mut input_stream: ScmStream = ScmStream::new_stdin();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-f" {
            match File::open(args.get(i + 1).unwrap()) {
                Ok(file) => {
                    input_stream = ScmStream::new_file(file);
                }
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                }
            }
        }

        if arg == "-i" {
            init = false;
        }
    }

    if init {
        match File::open("./init.scm") {
            Ok(file) => {
                input_stream = ScmStream::new_file(file);
                run(input_stream);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
        input_stream = ScmStream::new_stdin();
    }
    run(input_stream);
}

fn run(mut stream: ScmStream) {
    loop {
        let input: ScmObject = scm::reader::read(&mut stream);
        
        if let ObjectType::EOF = input.value {
            break;
        }
        let result = scm::eval::eval(input);
        scm::printer::print_result(result);
    }
}
