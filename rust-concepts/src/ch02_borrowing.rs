// This function demonstrates various aspects of Rust's borrowing system
pub fn demo() {
    println!("Demonstrating Rust borrowing!");

    // Example 1: Borrowing with references
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Example 2: Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);

    // Example 3: Multiple immutable references
    let s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {}, r2: {}", r1, r2);

    // Example 4: Mutable and immutable references (not simultaneously)
    let mut s3 = String::from("hello");
    {
        let r1 = &s3; // immutable borrow
        println!("r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference
    let r2 = &mut s3; // mutable borrow
    println!("r2: {}", r2);

    // Example 5: Dangling references
    let reference_to_nothing = no_dangle();
    println!("Reference: {}", reference_to_nothing);
}

// Make these functions public
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
