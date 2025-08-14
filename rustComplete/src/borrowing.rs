fn main(){
    let mut s = String :: from("hello");
    let r1 = &mut s;            //this would violate the first rule of borrowing , which says that
    let r2 = &mut s;            //we can only have one mutable refrence to the same data at a time

    println!("{}, {}", r1, r2);
}


//THIS WE CAN DO 
fn main(){
let mut s = String :: from ("Helllo");
{
    let r1 = &mut s;
}//r1 goes out of scope here , so we can make a new reference with 

let r2 = &mut s;
}

fn main(){
    let mut s = String :: from("hello");
    let r1 = &mut s;            
    let r2 = &mut s;  
    let r3 = &mut s;          

    println!("{}, {}, and {}", r1, r2, r3);
}