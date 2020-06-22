mod scm;
use std::env;
use std::fs::File;

fn main() {

    // unsafe {scm::memory::init_singeltons()};

    // unsafe {scm::memory::new_symbole(String::from("symbole: String"))};
    let args: Vec<String> = env::args().collect();

    let mut input_stream: scm::lib::ScmStream = scm::lib::ScmStream::new_stdin();

    for (i, arg) in args.iter().enumerate() {
        if arg == "-f" {
            match File::open(args.get(i + 1).unwrap()) {
                Ok (file)=> {
                    input_stream = scm::lib::ScmStream::new_file(file);
                }
                Err (e) => {
                    eprintln!("ERROR: {}", e);

                }
            }
        }
    }

    loop {
        let mut input: scm::lib::ScmObject = scm::reader::read(&mut input_stream);
        // let val: scm::lib::ScmObject = scm::eval::eval(&mut input);
        // scm::printer::print_result(val);

        scm::eval::eval(&mut input);
        scm::printer::print_result(input);
    }
}
