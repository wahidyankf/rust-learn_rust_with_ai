// This function demonstrates various aspects of Rust's lifetime system
pub fn demo() {
    println!("Demonstrating Rust lifetimes!");

    // Example 1: Basic lifetime annotations
    // Here we create two strings and pass references to them to the `longest` function
    // The `longest` function uses lifetime annotations to ensure the returned reference is valid
    let string1 = String::from("short");
    let string2 = String::from("longer string");
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);
    // Result: Longest string: longer string
    // The `result` reference is valid because it's tied to the lifetime of `string1` and `string2`
    // This demonstrates how Rust's borrow checker ensures that references are always valid

    // Example 2: Lifetime in struct definitions
    // We define a struct `ImportantExcerpt` that holds a reference to a string slice
    // The lifetime annotation ensures that the reference in the struct doesn't outlive the data it refers to
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("First sentence: {}", i.part);
    // Result: First sentence: Call me Ishmael
    // The `ImportantExcerpt` instance is valid as long as `novel` is valid
    // This shows how lifetimes can be used to ensure that struct fields referencing borrowed data remain valid

    // Example 3: Static lifetime
    // The 'static lifetime is a special lifetime that lasts for the entire duration of the program
    // String literals have a 'static lifetime by default
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
    // Result: Static string: I have a static lifetime.
    // This string will be available throughout the entire program's execution
    // 'static is useful for constants and other data that should live for the entire program runtime

    // Example 4: Lifetime elision
    // Rust has lifetime elision rules that allow us to omit lifetime annotations in common cases
    // Here, the compiler can infer the lifetimes without explicit annotations
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Result: The length of 'hello' is 5.
    // The `calculate_length` function doesn't need explicit lifetime annotations due to elision rules
    // This demonstrates how Rust's compiler can often infer lifetimes, reducing boilerplate code

    // Example 5: Lifetime bounds on generic types
    // We can use lifetime annotations with generic types to ensure references live long enough
    let x = 5;
    let y = 10;
    let point = Point { x: &x, y: &y };
    println!("Point coordinates: ({}, {})", point.x, point.y);
    // Result: Point coordinates: (5, 10)
    // The `Point` struct ensures that its references don't outlive the values they point to
    // This shows how lifetimes can be used with generic types to create flexible, safe abstractions

    // Example 6: Lifetime subtyping
    // Demonstrates how one lifetime can outlive another
    let s1 = String::from("short");
    let s2 = String::from("longer");
    let announce = Announcer {
        message: &s1,
        data: &s2,
    };
    println!(
        "Announcement: {} (data: {})",
        announce.message, announce.data
    );
    // This example shows how different lifetimes can be related to each other
    // The 'announce lifetime must outlive both 'message and 'data lifetimes

    // Example 7: Lifetime bounds on trait objects
    // Shows how to use lifetimes with trait objects
    let s = String::from("Hello, world!");
    let obj: Box<dyn PrintWithLifetime> = Box::new(LifetimePrinter { s: &s });
    obj.print();
    // This demonstrates how trait objects can use lifetimes
    // It allows for dynamic dispatch while still maintaining lifetime safety

    println!("🕰️ Time's up! The lifetime lesson is complete! ⏳🎉");
}

// This function demonstrates explicit lifetime annotations
// It takes two string slices and returns a reference to the longer one
// The lifetime 'a ensures that the returned reference is valid for at least as long as both input references
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// This function is a classic example of how lifetimes work in Rust
// By using the same lifetime 'a for both inputs and the output, we guarantee that:
// 1. Both input references live at least as long as 'a
// 2. The returned reference will not outlive either of the input references

// This struct demonstrates lifetimes in struct definitions
// The lifetime 'a ensures that the `part` reference doesn't outlive the data it refers to
pub struct ImportantExcerpt<'a> {
    pub(crate) part: &'a str,
}
// By including the lifetime 'a in the struct definition, we're saying:
// "An ImportantExcerpt instance cannot outlive the reference it holds in its `part` field"

// This function demonstrates lifetime elision rules
// Even though it takes a reference and returns a value derived from that reference,
// we don't need to specify lifetimes explicitly due to Rust's lifetime elision rules
pub fn calculate_length(s: &str) -> usize {
    s.len()
}
// Rust's lifetime elision rules allow us to omit explicit lifetime annotations in simple cases like this
// The compiler can infer that the input reference 's' and the return value don't need explicit lifetime annotations

// This struct demonstrates lifetime bounds on generic types
// It holds two references that might have different lifetimes
pub struct Point<'a, 'b> {
    pub(crate) x: &'a i32,
    pub(crate) y: &'b i32,
}
// This struct shows how we can use different lifetimes for different fields
// It allows for more flexible ownership patterns, where 'x' and 'y' can have independent lifetimes

// Implement a method for ImportantExcerpt
// This method doesn't need any additional lifetime annotations because it uses the lifetime from the impl block
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }
}
// This implementation demonstrates that methods on structs with lifetimes don't always need
// additional lifetime annotations. The lifetime 'a is already part of the type ImportantExcerpt<'a>

// This function demonstrates the 'static lifetime
// It returns a string slice with a 'static lifetime, which means it's available for the entire program run
#[allow(dead_code)]
pub fn static_lifetime() -> &'static str {
    "This string has a 'static lifetime"
}
// The 'static lifetime is special in Rust. It indicates that the reference is valid for the entire program run
// String literals are 'static by default, as they're stored in the program's binary

// This struct demonstrates lifetime subtyping
// The 'announce lifetime must outlive the 'message and 'data lifetimes
pub struct Announcer<'message, 'data> {
    pub(crate) message: &'message str,
    pub(crate) data: &'data str,
}
// This struct shows how we can relate different lifetimes to each other
// It's saying that the Announcer struct can't outlive either of its references

// This trait demonstrates lifetime bounds on trait objects
pub trait PrintWithLifetime<'a> {
    fn print(&self);
}
// This trait shows how we can use lifetimes with traits
// The lifetime 'a can be used to ensure that implementations of this trait
// don't outlive any references they might be holding

// This struct implements the PrintWithLifetime trait
pub struct LifetimePrinter<'a> {
    pub(crate) s: &'a str,
}

impl<'a> PrintWithLifetime<'a> for LifetimePrinter<'a> {
    fn print(&self) {
        println!("Printing with lifetime: {}", self.s);
    }
}
// This implementation shows how a struct with a lifetime can implement a trait with a lifetime
// It ensures that the LifetimePrinter doesn't outlive the string slice it's referencing
