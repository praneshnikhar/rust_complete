// fn main(){
//     let _f : bool = false;

//     let t: bool = false;
//     if !t{
//         println!("success!");
//     }
// }


// fn main(){
//     let f: bool =false;
//     let t: bool = true && false; //false
//     assert_eq!(t, f);

//     println!("Success!");
// }




// fn main() {
//     let v: (i32, i32) = (2, 3);
//     assert_eq!(v, implicitly_ret_tuple(2, 3));
//     println!("Success!");
// }

// fn implicitly_ret_tuple(x: i32, y: i32) -> (i32, i32) {
//     (x, y) 
// }

// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }



//what's the size of unit type

use std:: mem::size_of_val;
fn main(){
    let unit:() =();
    assert!(size_of_val(&unit) == 0);
    println!("Success");
}