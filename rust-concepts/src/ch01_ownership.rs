pub fn demo() {
    println!("Demonstrating Rust ownership!");

    // Example 1: Move semantics
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This would cause a compile error
    println!("s2: {}", s2);

    // Example 2: Clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    // Example 3: Copy for stack-only data
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Example 4: Ownership and functions
    let s5 = String::from("hello");
    takes_ownership(s5);
    // println!("{}", s5); // This would cause a compile error

    let x = 5;
    makes_copy(x);
    println!("x is still accessible: {}", x);

    // Example 5: Return values and scope
    let s6 = gives_ownership();
    println!("s6: {}", s6);

    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    println!("s8: {}", s8);
}

// This function takes ownership of the passed string
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called

// This function makes a copy of the passed integer
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope, but nothing special happens

// This function will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // This value is moved out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string // This value is returned and moves out to the calling function
}
