mod scm;

fn main() {
    loop {
        let input: scm::lib::ScmObject = scm::reader::read();
        let val: scm::lib::ScmObject = scm::eval::eval(input);
        scm::printer::print_result(val);
    }
}
