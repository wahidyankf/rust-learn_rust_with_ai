// This function demonstrates various aspects of Rust's borrowing system
pub fn demo() {
    println!("Demonstrating Rust borrowing!");

    // Example 1: Borrowing with references
    // Here we create a String and then borrow it immutably to calculate its length
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Result: The length of 'hello' is 5.
    // Note that s1 is still valid here because we only borrowed it immutably
    // This demonstrates how immutable borrowing allows us to use data without taking ownership

    // Example 2: Mutable references
    // We create a mutable String and then borrow it mutably to change its contents
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {}", s);
    // Result: After change: hello, world
    // The change is reflected in the original variable because we used a mutable reference
    // This shows how mutable borrowing allows us to modify data in-place

    // Example 3: Multiple immutable references
    // Rust allows multiple immutable references to the same data
    let s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {}, r2: {}", r1, r2);
    // Result: r1: hello, r2: hello
    // Both r1 and r2 are valid here because they're immutable references
    // This demonstrates Rust's ability to have multiple readers of the same data

    // Example 4: Mutable and immutable references (not simultaneously)
    // Rust enforces the rule that you can have either one mutable reference
    // or any number of immutable references, but not both at the same time
    let mut s3 = String::from("hello");
    {
        let r1 = &s3; // immutable borrow
        println!("r1: {}", r1);
        // Result: r1: hello
    } // r1 goes out of scope here, so we can make a new reference
    let r2 = &mut s3; // mutable borrow
    println!("r2: {}", r2);
    // Result: r2: hello
    // This is valid because the immutable borrow (r1) and the mutable borrow (r2) don't overlap
    // This example shows how Rust prevents data races by enforcing strict borrowing rules

    // Example 5: Preventing dangling references
    // Rust's borrow checker ensures we don't create dangling references
    let reference_to_nothing = no_dangle();
    println!("Reference: {}", reference_to_nothing);
    // Result: Reference: hello
    // This function returns a String, not a reference, avoiding a potential dangling reference
    // It demonstrates how Rust's ownership system prevents common memory safety issues

    // Example 6: Lifetime annotations
    // Sometimes Rust needs help understanding how long references should live
    let x = String::from("5");
    let y = String::from("10");
    let result = longest(&x, &y);
    println!("Longest string: {}", result);
    // Result: Longest string: 10
    // This example introduces lifetime annotations, which help the compiler understand
    // how long references should be valid

    // Example 7: Borrowing in loops
    let mut vec = vec![1, 2, 3, 4, 5];
    for i in &mut vec {
        *i *= 2;
    }
    println!("Doubled vector: {:?}", vec);
    // Result: Doubled vector: [2, 4, 6, 8, 10]
    // This demonstrates how we can borrow and modify elements in a collection
    // The `&mut` before `vec` creates an iterator that yields mutable references

    // Example 8: Self-referential structs (advanced topic)
    println!("Self-referential structs are an advanced topic in Rust.");
    println!("They often require special handling or crates like 'ouroboros'.");
    // Self-referential structs are structures that contain references to their own fields
    // They are challenging in Rust due to the borrow checker's rules and often require
    // advanced techniques or external crates to implement safely

    println!("🕺💃 Now, let's dance! The borrowing lesson is complete! 🎉🎊");
}

// This function borrows a String immutably and returns its length
pub fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len() // Return the length of the String
            // s goes out of scope here, but because it's a reference, it doesn't drop what it refers to
            // This function demonstrates how we can work with data without taking ownership
            // By using an immutable reference, we can read the String's length without moving it
}

// This function takes a mutable reference to a String and modifies it
pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // We can modify the String because we have a mutable reference to it
    // This function shows how mutable references allow us to change data in-place
    // Without taking ownership of the String, we can still modify its contents
}

// This function demonstrates how to avoid returning a dangling reference
pub fn no_dangle() -> String {
    let s = String::from("hello");
    s // We return s directly, transferring ownership to the caller
      // If we returned a reference to s, it would be a dangling reference
      // because s would be dropped at the end of this function
      // This function illustrates how Rust's ownership system prevents dangling references
      // By returning the String itself, we ensure that the caller gets a valid, owned value
}

// This function demonstrates lifetime annotations
pub fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    // The lifetime annotation 'a tells the compiler that the returned reference
    // will be valid for as long as both input references are valid
    // This ensures that the caller can't use the returned reference longer than the inputs
}
