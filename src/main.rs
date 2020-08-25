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
                run(ScmStream::new_file(file), &top_env);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
    }

    run(input_stream, &top_env);
}

fn run(mut stream: ScmStream, env: &ScmObject) {
    loop {
        let input: ScmObject = scm::reader::read(&mut stream);
        if let ScmObject::EOF = input {
            break;
        }
        let result = scm::eval::eval(input, &env);
        scm::printer::print_result(result);
    }
}

fn init_buildin(mut env: ScmObject) -> ScmObject{

    let mut fns = ScmObject::new_fn(BuildInFunction::FNPLUS, String::from("+"), 2);
    env = scm::environment::define_enviroment(env, ScmObject::new_symbol(String::from("+")), fns);

    fns = ScmObject::new_fn(BuildInFunction::QUOTE, String::from("'"), 1);
    env = scm::environment::define_enviroment(env, ScmObject::new_symbol(String::from("'")), fns);

    fns = ScmObject::new_fn(BuildInFunction::FNMINUS, String::from("-"), 2);
    env = scm::environment::define_enviroment(env, ScmObject::new_symbol(String::from("-")), fns);

    env
}
