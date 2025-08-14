// fn main(){
// let s= String :: from("hello world");
// let hello = &s[0..5];
// let world= &s[6..1];

// }


fn main(){
    let s: Box <str> = "hello, world".into(); // or just -> let s: &str ="hello , world";
    greetings(&s)                             //greetings(s) 
}

fn greetings(s: &str){
    println!("{}", s)
}