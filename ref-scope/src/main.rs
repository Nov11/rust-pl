fn main() {
    let mut s = String::from("string");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);

    //not that same with variable scope
    //r1/r2's scope is not used below, their scopes end and it's valid to create mutable ref.
    let r3 = &mut s; // BIG PROBLEM
    println!("r3 {}", r3);
}
