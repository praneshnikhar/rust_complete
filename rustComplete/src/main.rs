fn main(){
    // println!("Hello world");
    let x: i32 = 5;
    let y: i32 =10;


    assert_eq!(x,5);
    println!("success!");

    let mut x =1;
    x = x+2; //3
    x = 7;

    //shadowing and re-binding
    let x = x;
    // x = x + 3; this will give error of immutable
    //so do- 
    let mut x = x;
    x =x +3;
    let y:i32 =4;
    //shadowing
    let y: &str = "I can also be text!"
    ;



    assert_eq!(x,3);
    println!("Success!");

    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is{}  and the value of y is {}", x, y);
    }
    println!("the value of x is {} the value of y is{}",x,y); //y -> out of scope

}

