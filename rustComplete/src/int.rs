fn main(){
    let v1: u16 = 251_u16 +8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    assert!(0.2 as f32 + 0.3_f32 == 0.5_f32);
    println!("Success");
}