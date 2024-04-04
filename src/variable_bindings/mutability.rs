fn main() {
    let _inmutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    //Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    //Error! Cannot assign a new value to an inmutable variable
    // _inmutable_binding += 1;
}
