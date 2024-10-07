// This function demonstrates various aspects of Rust's ownership system
pub fn demo() {
    println!("Demonstrating Rust ownership!");

    // Example 1: Move semantics
    // In Rust, when we assign a value to another variable, the ownership is moved
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This would cause a compile error because s1's ownership has been moved to s2
    println!("s2: {}", s2);
    // Result: s2: hello

    // Example 2: Clone
    // If we want to create a deep copy of the data, we can use the clone method
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    // Result: s3: world, s4: world
    // Both s3 and s4 are valid here because we created a new copy of the data

    // Example 3: Copy for stack-only data
    // For simple types that have a known size at compile time, the Copy trait is implemented
    // This means that the value is copied instead of moved when assigned or passed to a function
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Result: x: 5, y: 5
    // Both x and y are valid here because integers implement the Copy trait

    // Example 4: Ownership and functions
    // When we pass a value to a function, the ownership is transferred to that function
    let s5 = String::from("hello");
    takes_ownership(s5);
    // Result: hello
    // println!("{}", s5); // This would cause a compile error because s5's ownership has been moved to the function

    // For types that implement Copy, the value is copied instead of moved
    let x = 5;
    makes_copy(x);
    // Result: 5
    println!("x is still accessible: {}", x);
    // Result: x is still accessible: 5

    // Example 5: Return values and scope
    // Functions can also transfer ownership of their return values
    let s6 = gives_ownership();
    println!("s6: {}", s6);
    // Result: s6: yours

    // We can also take ownership of a value, do something with it, and then return ownership
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    println!("s8: {}", s8);
    // Result: s8: hello
    // s7 is no longer valid here, but s8 is
}

// This function takes ownership of the passed string
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called, freeing the memory

// This function makes a copy of the passed integer
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope, but nothing special happens because it's a Copy type

// This function will move its return value into the function that calls it
pub fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // This value is moved out to the calling function
}

// This function takes a String and returns one
pub fn takes_and_gives_back(a_string: String) -> String {
    a_string // This value is returned and moves out to the calling function
}
