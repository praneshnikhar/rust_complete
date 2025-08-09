// fn main(){
//     let x: u32 = 5u32;

//     let y ={
//         let x_squared = x*x;
//         let x_cube = x_squared * x;
        
//         //this expression will be asigned to 'y'
//         x_cube +x_squared +x
//     };

//     let z: u32 = {
//         //the semicolon suppresses this expression and '()' is assigned to 'z'
//         2 * x
//     };
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }


// //2.
// fn main(){
//     let v :() = {
//         let mut x =1;
//         x+=2
//     };

//     assert_eq!(v,());
//     println!("Successs!");
// }



//3.
fn main(){
    let s:i32 = sum(1, 2);//3
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y:i32) -> i32 {
    x+y
}