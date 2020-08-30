mod scm;

use scm::environment::ScmEnvironment;
use scm::scm_object::{BuildInFunction, BuildInSyntax, NumArgs, ScmObject};
use scm::stream::ScmStream;

use std::env;
use std::fs::File;
use std::time::SystemTime;

fn main() {
    scm::selftest::selftest();
    let mut init: bool = false;
    let mut show_time: bool = false;
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

        if arg == "-t" {
            show_time = true;
        }
    }

    let mut top_env: ScmEnvironment = ScmEnvironment::new();
    init_build_in(&mut top_env);

    if init {
        match File::open("./init.scm") {
            Ok(file) => {
                top_env = run(ScmStream::new_file(file), top_env, false);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
    }

    run(input_stream, top_env, show_time);
}

fn run(mut stream: ScmStream, mut env: ScmEnvironment, show_time: bool) -> ScmEnvironment {
    loop {
        let input: ScmObject = scm::reader::read(&mut stream);
        if let ScmObject::EndOfFile = input {
            break;
        }
        let start = SystemTime::now();
        let re = scm::teval::eval(input, env);
        let stop = SystemTime::now();
        env = re.1;
        match re.0 {
            ScmObject::Void => {}
            _ => {
                scm::printer::print_result(re.0);
                println!();
            }
        }
        if show_time {
            println!("exec time: {:?}", stop.duration_since(start));
        }
    }
    env
}

fn init_build_in(env: &mut ScmEnvironment) {
    env.define(
        ScmObject::Symbol(String::from("quote")),
        &ScmObject::new_syntax(BuildInSyntax::Quote, String::from("Quota"), 1),
    );
    env.define(
        ScmObject::Symbol(String::from("define")),
        &ScmObject::new_syntax(BuildInSyntax::Define, String::from("define"), 2),
    );
    env.define(
        ScmObject::Symbol(String::from("set")),
        &ScmObject::new_syntax(BuildInSyntax::Set, String::from("set"), 2),
    );
    env.define(
        ScmObject::Symbol(String::from("lambda")),
        &ScmObject::new_syntax(BuildInSyntax::Lambda, String::from("lambda"), 2),
    );
    env.define(
        ScmObject::Symbol(String::from("if")),
        &ScmObject::new_syntax(BuildInSyntax::If, String::from("if"), 3),
    );
    env.define(
        ScmObject::Symbol(String::from("begin")),
        &ScmObject::new_syntax(BuildInSyntax::Begin, String::from("begin"), 3),
    );

    env.define(
        ScmObject::Symbol(String::from("+")),
        &ScmObject::new_fn(
            BuildInFunction::Plus,
            String::from("Plus"),
            NumArgs::Unlimited as i64,
        ),
    );
    env.define(
        ScmObject::Symbol(String::from("-")),
        &ScmObject::new_fn(
            BuildInFunction::Minus,
            String::from("Minus"),
            NumArgs::Unlimited as i64,
        ),
    );
    env.define(
        ScmObject::Symbol(String::from("display")),
        &ScmObject::new_fn(
            BuildInFunction::Display,
            String::from("display"),
            NumArgs::Unlimited as i64,
        ),
    );
    env.define(
        ScmObject::Symbol(String::from("print")),
        &ScmObject::new_fn(
            BuildInFunction::Print,
            String::from("print"),
            NumArgs::Unlimited as i64,
        ),
    );
    env.define(
        ScmObject::Symbol(String::from("print-env")),
        &ScmObject::new_fn(
            BuildInFunction::PrintEnv,
            String::from("print-env"),
            0,
        ),
    );
}
