mod scm;

fn main() {
    loop {
        let _input: Option<scm::lib::ScmObject> = scm::reader::read();
        //println!("reade done
        match _input { 
            Some (x) => {
                let _val: scm::lib::ScmObject = scm::eval::eval(x);
                //println!("eval done");
                scm::printer::print(_val);
            }
            None => {
                continue;
            }
        }
        
    }
}
