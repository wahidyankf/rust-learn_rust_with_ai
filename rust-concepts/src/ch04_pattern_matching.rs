// This function demonstrates various aspects of Rust's pattern matching
pub fn demo() {
    println!("Demonstrating Rust pattern matching!");

    // Example 1: Basic match expression
    let number = 13;
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values using the | (or) operator
        // This allows us to specify multiple patterns in a single match arm
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range using ..=
        // This matches any value from 13 to 19, inclusive
        13..=19 => println!("A teen"),
        // Handle all other cases with the wildcard pattern _
        // This is a catch-all pattern that matches any value not matched by previous patterns
        _ => println!("Ain't special"),
    }
    // Result: A teen

    // Example 2: Matching with guards
    // Guards allow for additional boolean conditions in match arms
    let pair = (2, -2);
    match pair {
        // Check if both components are equal
        // The guard 'if x == y' adds an additional condition to the pattern
        (x, y) if x == y => println!("These are twins"),
        // Check if the sum of components is zero
        // This demonstrates how we can use complex expressions in guards
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        // Check if the first component is odd
        // The underscore _ in (x, _) means we don't care about the second value
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        // Default case
        // This catches any pair that doesn't match the above patterns
        _ => println!("No correlation..."),
    }
    // Result: Antimatter, kaboom!

    // Example 3: Destructuring structs
    // Define a simple Point struct with x and y coordinates
    struct Point {
        x: i32,
        y: i32,
    }
    // Create a new Point instance
    let point = Point { x: 0, y: 7 };
    // Destructure the point into its components
    // This creates two new variables, x and y, with the values from point
    let Point { x, y } = point;
    // Verify that the destructuring worked correctly
    assert_eq!(0, x);
    assert_eq!(7, y);
    // Match on different configurations of the Point struct
    match point {
        // Match points on the x-axis (y = 0)
        // This demonstrates how we can match on specific field values
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Match points on the y-axis (x = 0)
        // Similar to the previous arm, but matching on x = 0
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // Match any other point
        // This is a catch-all for points not on either axis
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // Result: On the y axis at 7

    // Example 4: Matching enums
    // Define an enum with different variants
    // This enum represents different types of messages in a system
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // Create an instance of the Message enum
    let msg = Message::ChangeColor(0, 160, 255);
    // Match on different enum variants
    match msg {
        // Simple variant with no data
        Message::Quit => println!("Quit"),
        // Struct-like variant: destructure to access fields
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        // Tuple-like variant: extract the String
        Message::Write(text) => println!("Text message: {}", text),
        // Tuple-like variant: destructure to access color components
        Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
    }
    // Result: Change color to r: 0, g: 160, b: 255

    // Example 5: Pattern matching with Option
    // Option is an enum that represents the presence or absence of a value
    let some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
    // Match on the Option enum
    match some_number {
        // If the Option contains a value, bind it to 'i'
        Some(i) => println!("Got an integer: {}", i),
        // If the Option is None, execute this arm
        None => println!("No integer!"),
    }
    // Result: Got an integer: 5

    // Example 6: if let expressions
    // if let is a shorter way to match on a single pattern
    // It's particularly useful when you only care about one specific pattern
    if let Some(i) = some_number {
        println!("Matched {}", i);
    }
    // Result: Matched 5

    // Example 7: while let conditional loops
    // while let allows a loop to continue as long as a pattern matches
    // This is useful for iterating over values that implement the Iterator trait
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // Pop items from the stack while it's not empty
    // This demonstrates how while let can be used to process a sequence of values
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // Result:
    // 3
    // 2
    // 1

    println!("🧩 Puzzle solved! The pattern matching lesson is complete! 🎭🎉");
}
