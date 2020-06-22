mod scm;

fn main() {
    let readfile: bool = false;
    let mut input_stream: scm::lib::ScmStream = scm::lib::ScmStream::new_stdin();

    loop {
        if !readfile {

        }

        let input: scm::lib::ScmObject = scm::reader::read(&mut input_stream);
        let val: scm::lib::ScmObject = scm::eval::eval(input);
        scm::printer::print_result(val);
    }
}
