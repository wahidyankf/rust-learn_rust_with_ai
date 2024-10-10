// This function demonstrates various aspects of Rust's ownership system
pub fn demo() {
    println!("Demonstrating Rust ownership!");

    // Example 1: Move semantics
    // In Rust, when we assign a value to another variable, the ownership is moved
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This would cause a compile error because s1's ownership has been moved to s2
    // The line above is commented out because it would cause a compile-time error.
    // After the move, s1 is no longer valid and attempting to use it would result in a "use of moved value" error.
    println!("s2: {}", s2);
    // Result: s2: hello

    // Example 2: Clone
    // If we want to create a deep copy of the data, we can use the clone method
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    // Result: s3: world, s4: world
    // Both s3 and s4 are valid here because we created a new copy of the data
    // The clone method creates a new allocation in memory with the same contents,
    // allowing both variables to own their own independent data.

    // Example 3: Copy for stack-only data
    // For simple types that have a known size at compile time, the Copy trait is implemented
    // This means that the value is copied instead of moved when assigned or passed to a function
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Result: x: 5, y: 5
    // Both x and y are valid here because integers implement the Copy trait
    // Types that implement Copy are duplicated rather than moved, so the original variable remains valid.

    // Example 4: Ownership and functions
    // When we pass a value to a function, the ownership is transferred to that function
    let s5 = String::from("hello");
    takes_ownership(s5);
    // Result: hello
    // println!("{}", s5); // This would cause a compile error because s5's ownership has been moved to the function
    // After calling takes_ownership, s5 is no longer valid in this scope.
    // The ownership of the String has been transferred to the function.

    // For types that implement Copy, the value is copied instead of moved
    let x = 5;
    makes_copy(x);
    // Result: 5
    println!("x is still accessible: {}", x);
    // Result: x is still accessible: 5
    // Since i32 implements Copy, x remains valid after being passed to makes_copy.

    // Example 5: Return values and scope
    // Functions can also transfer ownership of their return values
    let s6 = gives_ownership();
    println!("s6: {}", s6);
    // Result: s6: yours
    // s6 now owns the String returned by gives_ownership

    // We can also take ownership of a value, do something with it, and then return ownership
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    println!("s8: {}", s8);
    // Result: s8: hello
    // s7 is no longer valid here, but s8 is
    // The ownership of the String has been transferred from s7 to the function, then back to s8.
}

// This function takes ownership of the passed string
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called, freeing the memory
  // The String's memory is automatically freed when some_string goes out of scope.

// This function makes a copy of the passed integer
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope, but nothing special happens because it's a Copy type
  // No special cleanup is needed for Copy types when they go out of scope.

// This function will move its return value into the function that calls it
pub fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // This value is moved out to the calling function
                // The ownership of some_string is transferred to the calling function.
                // No need for an explicit return statement; the last expression is implicitly returned.
}

// This function takes a String and returns one
pub fn takes_and_gives_back(a_string: String) -> String {
    a_string // This value is returned and moves out to the calling function
             // The function takes ownership of a_string when it's called,
             // and then transfers that ownership back to the calling function when it returns.
}
