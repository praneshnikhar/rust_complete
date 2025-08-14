fn main(){
    let reference_to_nothing = dangle();
}                                               


fn dangle() -> &String {
    let s = String :: from("hello");    //this will violate the second rule which states that references must always be valid
    &s
}

fn no_dangle() ->String{
    let s = String :: from("hello");
    s
}