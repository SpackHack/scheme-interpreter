#![feature(get_mut_unchecked)]

mod scm;

use scm::environment::ScmEnvironment;
use scm::scm_object::{BuildInFunction, BuildInSyntax, NumArgs, ScmObject};
use scm::stream::{ScmStream, Stream};

use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;
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

    let mut top_env: ScmObject = ScmObject::new_env();
    init_build_in(&mut top_env);

    if init {
        match File::open("./init.scm") {
            Ok(file) => {
                run(ScmStream::new_file(file), top_env.clone(), false);
            }
            Err(err) => {
                println!("ERR in read init: {}", err);
            }
        }
    }

    run(input_stream, top_env, show_time);
}

fn run(mut stream: ScmStream, mut env: ScmObject, show_time: bool) {
    loop {
        if let Stream::STDIN(_) = stream.stream {
            print!("> ");
            io::stdout().flush().unwrap();
        }
        let input: ScmObject = scm::reader::read(&mut stream);

        if let ScmObject::EndOfFile = input {
            break;
        }
        let start = SystemTime::now();
        let evaluiert = scm::teval::eval(input, env.clone());
        let stop = SystemTime::now();

        scm::printer::print_result(evaluiert);

        if show_time {
            println!("exec time: {:?}", stop.duration_since(start));
        }
    }
}

fn init_build_in(scm_object: &mut ScmObject) {
    let mut rc_env = scm_object.get_env();
    unsafe {
        let env = Rc::get_mut_unchecked(&mut rc_env);

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
            &ScmObject::new_fn(BuildInFunction::PrintEnv, String::from("print-env"), 0),
        );
        env.define(
            ScmObject::Symbol(String::from("*")),
            &ScmObject::new_fn(
                BuildInFunction::Times,
                String::from("*"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("cons")),
            &ScmObject::new_fn(BuildInFunction::Cons, String::from("cons"), 2),
        );
        env.define(
            ScmObject::Symbol(String::from("car")),
            &ScmObject::new_fn(BuildInFunction::Car, String::from("car"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("cdr")),
            &ScmObject::new_fn(BuildInFunction::Cdr, String::from("cdr"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("eq")),
            &ScmObject::new_fn(BuildInFunction::Equal, String::from("equal"), 2),
        );
        env.define(
            ScmObject::Symbol(String::from(">")),
            &ScmObject::new_fn(BuildInFunction::Gt, String::from("Gt"), 2),
        );
        env.define(
            ScmObject::Symbol(String::from("is-string")),
            &ScmObject::new_fn(BuildInFunction::IsChars, String::from("is-string"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("is-cons")),
            &ScmObject::new_fn(BuildInFunction::IsCons, String::from("is-cons"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("is-number")),
            &ScmObject::new_fn(BuildInFunction::IsNumber, String::from("is-number"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("is-buildinfn")),
            &ScmObject::new_fn(BuildInFunction::IsFunction, String::from("is-function"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("is-syntax")),
            &ScmObject::new_fn(BuildInFunction::IsSyntax, String::from("is-syntax"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("is-fn")),
            &ScmObject::new_fn(
                BuildInFunction::IsUserFunctions,
                String::from("is-user_function"),
                1,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("=")),
            &ScmObject::new_fn(
                BuildInFunction::EqualNumber,
                String::from("equal Number"),
                2,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("fn-body")),
            &ScmObject::new_fn(BuildInFunction::FnBody, String::from("fn body"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("fn-arg")),
            &ScmObject::new_fn(BuildInFunction::FnArg, String::from("fn arg"), 1),
        );
        env.define(
            ScmObject::Symbol(String::from("list")),
            &ScmObject::new_fn(
                BuildInFunction::List,
                String::from("list"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("load")),
            &ScmObject::new_fn(
                BuildInFunction::Load,
                String::from("load"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("open")),
            &ScmObject::new_fn(
                BuildInFunction::Open,
                String::from("open"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("close")),
            &ScmObject::new_fn(
                BuildInFunction::Close,
                String::from("close"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("read")),
            &ScmObject::new_fn(
                BuildInFunction::Read,
                String::from("read"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("read-char")),
            &ScmObject::new_fn(
                BuildInFunction::ReadChar,
                String::from("read-char"),
                NumArgs::Unlimited as i64,
            ),
        );
        env.define(
            ScmObject::Symbol(String::from("read-line")),
            &ScmObject::new_fn(
                BuildInFunction::ReadLine,
                String::from("read-line"),
                NumArgs::Unlimited as i64,
            ),
        );
    }
}
