use std :: mem ::size_of_val;

// fn main(){
//     let c1 : char = 'a'; //size of c1 = 4 bytes
//     // assert_eq!(size_of_val(&c1),1);

//     println!("{}", size_of_val(&c1));

//     // let c2 : char ='+';
//     // assert_eq!(size_of_val(&c2),3);

//     println!("Success");
// }

fn main(){
    let c1: char = 'c';
    print_char(c1);
}
fn print_char(c:char){
    println!("{}", c);
    
}