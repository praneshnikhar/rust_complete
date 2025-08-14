// fn main(){
//     let x : i32 = 5;
//     let p : &i32 = &x;
//     println!("the memory address of x is {:p}", p);
// }


// fn main(){
//      let x: i32 = 5;
//      let y : &i32 = &x;
                                    //checks if the pointer y points to the address of the variable x with value of 5
//      assert_eq!(5, *y);
//      println!("Success!");
// }


// fn main(){
//     let mut s: String = String :: from ("hello, ");
//     borrow_object(&s);
//     println!("Success!");
// }
// fn borrow_object(s:&String){}


//error fixed ober here
// fn main(){
//     let mut s:String = String :: from("hello, ");
//     push_str(&mut s);
//     println!("Success!");
// }

// fn push_str(s: &mut String){
//     s.push_str("world")
// }


// fn main(){
//     let mut s: String = String :: from("hello, ");
//     let p: &mut String= &mut s;

//     p.push_str("world");
//     println!("Success!");
// }


//"ref" CAN BE USED TO Take references to a value , similar to &.
// fn main(){
//     let c :char = 'a';
//     let r1: &char = &c;
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);

//     assert_eq!(get_addr(r1), get_addr(r2));
//     println!("Success!");
// }

// fn get_addr(r: &char) -> String{
//     format!("{:p}", r)
// }


//BORROWING RULES

fn main(){
    let mut s:String = String:: from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{},{}, r1, r2");
    println!("Success!");
}