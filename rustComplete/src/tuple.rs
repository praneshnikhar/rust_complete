fn main(){
    // let(mut x,y) = (1,2);
    // x+=2; 

    let(x,y);
    (x,..) = (3,4);
    [..,y] = [1,2];

    assert_eq!([x,y], [3,2]);

    assert_eq!(x,3);
    assert_eq!(y,2);
    println!("Success!");

}