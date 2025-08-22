// fn main(){
// let s= String :: from("hello world");
// let hello = &s[0..5];
// let world= &s[6..1];

// }


// fn main(){
//     let s: Box <str> = "hello, world".into(); // or just -> let s: &str ="hello , world";
//     greetings(&s)                             //greetings(s) 
// }
// fn greetings(s: &str){
//     println!("{}", s)
// }


// fn main(){
//     let mut s :String = String :: from ("hello");
//     s.push(',');
//     s.push_str("world");
//     s+= "!";

//     println!("{}", s); 
// }



// fn main(){
//     let s:String= String::from("i like dogs");
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "i like catds");
//     println!("Success");
// }


fn main(){
    let s1:String = String::from("hello, ");
    let s2:String = String::from("world!");
    let s3:String = s1+s2.as_str();
    assert_eq!(s3, "hello , wolrd!");
    println!("{}", s3);
}