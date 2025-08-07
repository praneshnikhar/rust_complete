fn main(){
    // let(mut x,y) = (1,2);
    // x+=2; 


    //integers are by default i32

    // let(x,y);
    // (x,..) = (3,4);
    // [..,y] = [1,2];

    // assert_eq!([x,y], [3,2]);  -> these all codes will give error instead replace assert_eq!.. like at the bottom

    // assert_eq!(x,3);
    // assert_eq!(y,2);
    // println!("Success!");

    // let v: u16 = 38_u8 as u16;
    // println!("success")


    //assert_eq..

    let x: u32 =5;
    assert_eq!("u32".to_string(),type_of(&x));
    println!("successs!!")

    
}

fn type_of<T>(_: &T) -> String{
    format!("{}",std::any::type_name::<T>()) //i32
}