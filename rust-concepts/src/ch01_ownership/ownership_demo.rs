pub fn demonstrate_ownership() {
    // Demonstrating ownership and borrowing
    let original_string = String::from("Hello");

    println!("Original string: {}", original_string);

    // Demonstrating ownership transfer
    let owned_string = takes_ownership(original_string);
    println!("After ownership transfer: {}", owned_string);

    // Creating a new string for further demonstrations
    let mut new_string = String::from("World");

    // Demonstrating borrowing
    borrows_string(&new_string);
    println!("After borrowing: {}", new_string);

    // Demonstrating mutable borrowing
    mutably_borrows_string(&mut new_string);
    println!("After mutable borrowing: {}", new_string);
}

fn takes_ownership(some_string: String) -> String {
    println!("Took ownership of: {}", some_string);
    some_string // Return the string to demonstrate ownership transfer
}

fn borrows_string(some_string: &String) {
    println!("Borrowed: {}", some_string);
}

fn mutably_borrows_string(some_string: &mut String) {
    some_string.push_str(" (mutably borrowed)");
}
