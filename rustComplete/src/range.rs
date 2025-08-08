
// fn main(){
//     let mut sum: i32 =0;
    
//     for i in -3..2{
//         sum += i
//     }

//     assert!(sum == -5);
//     for c in 'a'..='z'{
//         println!("{}",c);
//     }

//     //for ascii code 
//     for c in 'a'..='z'{
//         println!("{}",c as u8); //write as u8
//     }

// }

use std::ops::{Range, RangeInclusive};
fn main(){
    assert_eq!((1..5), Range{start:1, end:5});
    assert_eq!((1..= 5),RangeInclusive::new(1,5));

    println!("Success!");
}