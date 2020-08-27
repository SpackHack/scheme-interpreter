mod scm;

use scm::scmObject::{ScmObject, BuildInSyntax, BuildInFunction};
use scm::stream::ScmStream;
use scm::environment::ScmEnvironment;
use std::env;
use std::fs::File;

fn main() {
    scm::selftest::selftest();
    let mut init: bool = false;
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
            init = true;
        }
    }

    let mut top_env: ScmEnvironment = ScmEnvironment::new();
    init_build_in(&mut top_env);

    if init {
        match File::open("./init.scm") {
            Ok(file) => {
                run(ScmStream::new_file(file), &mut top_env);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
    }

    run(input_stream, &mut top_env);
}

fn run(mut stream: ScmStream, mut env: &mut ScmEnvironment) {
    loop {
        let input: ScmObject = scm::reader::read(&mut stream);
        if let ScmObject::EOF = input {
            break;
        }
        let evaluiert = scm::eval::eval(input, &mut env);
        scm::printer::print_result(evaluiert);
    }
}

fn init_build_in(env: &mut ScmEnvironment) {
    env.define(
        ScmObject::SYMBOL(String::from("quota")),
        ScmObject::new_syntax(BuildInSyntax::Quote, String::from("Syntax Quota"), 1),
    );
    env.define(
        ScmObject::SYMBOL(String::from("define")),
        ScmObject::new_syntax(BuildInSyntax::Define, String::from("Syntax define"), 2),
    );
    env.define(
        ScmObject::SYMBOL(String::from("set")),
        ScmObject::new_syntax(BuildInSyntax::Set, String::from("Syntax set"), 2),
    );
    env.define(
        ScmObject::SYMBOL(String::from("lambda")),
        ScmObject::new_syntax(BuildInSyntax::Lambda, String::from("Syntax lambda"), 2),
    );

    env.define(
        ScmObject::SYMBOL(String::from("+")),
        ScmObject::new_fn(BuildInFunction::Plus, String::from("FN Plus"), 2),
    );
    env.define(
        ScmObject::SYMBOL(String::from("-")),
        ScmObject::new_fn(BuildInFunction::Minus, String::from("-"), 2),
    );
}
