fn main(){
    println!("success!");
}

fn get_option(tp: u8) -> Option <i32>{
    match tp{ //switch
        1 => {

        }
        _ => { //if there is any other case

        }
        
    };
    never_return_fn()
   
}
fn never_return_fn() -> !{
    panic!()
    // unimplemented()       //you can use this when the function is not implemented
    // todo!()  //macro
}