use super::environment::*;
use super::scm_object::*;
use super::stack::*;

static mut STACK: Stack<ScmObject> = Stack::new(100);
static mut RETURN_STACK: Vec<ScmObject> = Vec::new();

static mut return_value: ScmObject = ScmObject::NIL;

pub struct ReturnFunction {
    pub func: fn()  -> Option<ReturnFunction>,
}

pub fn eval(input: ScmObject, mut env: &mut ScmEnvironment) -> ScmObject {
    unsafe{ STACK.push(input) };
    return trampolin(t_eval);
}

fn trampolin(function_ptr:  fn() -> Option<ReturnFunction>) -> ScmObject {

    unsafe { RETURN_STACK.push(ScmObject::None) };

    let mut next_function_ptr: Option<ReturnFunction> = Some(ReturnFunction{func:function_ptr});

    while let Some(f) = next_function_ptr {
        next_function_ptr = (f.func)();
    }
    
    return unsafe{return_value.clone() };
}

pub fn t_eval() -> Option<ReturnFunction> {
    
    let expression: ScmObject = unsafe{ STACK.pop() };
    let env: ScmObject;

    match expression {
        ScmObject::SYMBOL(symbol) => {

        }
        _ => {}
    }


    return None;
    //return Some(ReturnFunction{func: t_eval});
}
