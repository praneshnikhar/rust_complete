// fn main(){
//     let s = String :: from("Hello");
//     takes_ownership(s);
//     let x = 5;
//     makes_copy(x);
// }

// fn takes_ownership(some_string:String){
//     println!("{}", some_string);
// }
// fn makes_copy(some_integer:i32){
//     println!("{}", some_integer);
// }



fn main(){
    let s1 = gives_ownership(); //gives_ownership moves it return value into s1
    let s2= String :: from("hello"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back , which also
                                        //moves its return value into s3 
} //here s3 goes out of scope and is dropped , s2 was moved , so nothing happens. s1 goes out of scope and is dropped

fn gives_ownership()-> String{                      //gives_ownership will moves its return value into the function that calls it
    let some_string = String :: from("yours");      // some strings come into scope
    some_string                                     //some string is returned and moves out to the calling function
}
fn takes_and_gives_back(a_string:String)-> String{ //a_String comes into scope
    a_string        //a_strng is reutrned and movws out to the calling function
}