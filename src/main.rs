mod scm;

fn main() {
    loop {
        let _input: Option<scm::lib::ScmObject> = scm::reader::read();
        match _input { 
            Some (x) => {
                let _val: scm::lib::ScmObject = scm::eval::eval(x);
                scm::printer::print(_val);
            }
            None => {
                continue;
            }
        }
        
    }
}
