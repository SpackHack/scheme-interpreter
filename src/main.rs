mod scm;
use scheme_lib::*;
use std::env;
use std::fs::File;

fn main() {
    //scm::selftest::selftest();
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

    let mut top_env: ScmObject = ScmObject::new_env();
    top_env = init_buildin(top_env);

    if init {
        match File::open("./init.scm") {
            Ok(file) => {
                top_env = run(ScmStream::new_file(file), top_env);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
    }

    run(input_stream, top_env);
}

fn run(mut stream: ScmStream, env: ScmObject) -> ScmObject{
    let mut e = env;
    loop {
       
        let input: ScmObject = scm::reader::read(&mut stream);
        if let ScmObject::EOF = input {
            break;
        }
        let evaluiert = scm::eval::eval(input, e);
        e = evaluiert.1;
        scm::printer::print_result(evaluiert.0);
    }
    e
}

fn init_buildin(mut env: ScmObject) -> ScmObject {
    env = scm::environment::define_enviroment(
        env,
        ScmObject::new_symbol(String::from("quota")),
        ScmObject::new_syntax(BuildInSyntax::Quote, String::from("Syntax Quota"), 1),
    );

    env = scm::environment::define_enviroment(
        env,
        ScmObject::new_symbol(String::from("define")),
        ScmObject::new_syntax(BuildInSyntax::Define, String::from("Syntax define"), 2),
    );

    env = scm::environment::define_enviroment(
        env,
        ScmObject::new_symbol(String::from("+")),
        ScmObject::new_fn(BuildInFunction::Plus, String::from("FN Plus"), 2),
    );

    env = scm::environment::define_enviroment(
        env,
        ScmObject::new_symbol(String::from("-")),
        ScmObject::new_fn(BuildInFunction::Minus, String::from("-"), 2),
    );

    env
}
