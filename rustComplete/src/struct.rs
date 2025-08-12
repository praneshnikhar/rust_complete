fn main(){

    struct Person{
        name :String,
        age: Box <u8>,
    }
    let person: Person = Person{
        name:String::from("Alice"), 
        age : Box ::new(20),
    };

    let Person{name , ref age} =person;

    println!("the person's age is {}", age);
    println!("the person's name is {}", name);
    
} 