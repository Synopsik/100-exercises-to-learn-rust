use std::ops::Add;

fn main() {
    move_semantics();
    ownership_transfer_into_functions();
    returning_ownership();
    mutable_vs_immutable_borrowing()
    
}


fn move_semantics() {
    let s1 = String::from("hello");
    let s2 = s1.clone().add(" world");
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn ownership_transfer_into_functions() {
    let s = String::from("ownership");
    take_ownership(&s);
    
    println!("Still have s: {}", s);
    
    fn take_ownership(input: &String) {
        println!("Got: {}", input);
    }
}

fn returning_ownership() {
    let s1 = String::from("world");
    let s2 = give_back(&s1);
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    
    fn give_back(input: &String) -> &String {
        input
    }
}

fn mutable_vs_immutable_borrowing() {
    let mut name = String::from("Alice");
    
    let r1 = &name;
    println!("Immutable: {}", r1);
    
    let r2 = &mut name.add(" Berry");
    println!("Mutable: {}", r2);
}