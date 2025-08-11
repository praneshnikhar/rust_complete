fn main(){
    let x : Box<i32>= Box ::new(5);

    let y : Box<i32>  =  Box  ::new(1);//implement this like , dont change other lines
     
    *y = 4;

    assert_eq!(*x,5); //0x395604...

    println!("Success!");
}