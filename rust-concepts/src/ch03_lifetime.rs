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

    // Example 3: Static lifetime
    // The 'static lifetime is a special lifetime that lasts for the entire duration of the program
    // String literals have a 'static lifetime by default
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
    // Result: Static string: I have a static lifetime.
    // This string will be available throughout the entire program's execution

    // Example 4: Lifetime elision
    // Rust has lifetime elision rules that allow us to omit lifetime annotations in common cases
    // Here, the compiler can infer the lifetimes without explicit annotations
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Result: The length of 'hello' is 5.
    // The `calculate_length` function doesn't need explicit lifetime annotations due to elision rules

    // Example 5: Lifetime bounds on generic types
    // We can use lifetime annotations with generic types to ensure references live long enough
    let x = 5;
    let y = 10;
    let point = Point { x: &x, y: &y };
    println!("Point coordinates: ({}, {})", point.x, point.y);
    // Result: Point coordinates: (5, 10)
    // The `Point` struct ensures that its references don't outlive the values they point to

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

    // Example 7: Lifetime bounds on trait objects
    // Shows how to use lifetimes with trait objects
    let s = String::from("Hello, world!");
    let obj: Box<dyn PrintWithLifetime> = Box::new(LifetimePrinter { s: &s });
    obj.print();

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

// This struct demonstrates lifetimes in struct definitions
// The lifetime 'a ensures that the `part` reference doesn't outlive the data it refers to
pub struct ImportantExcerpt<'a> {
    pub(crate) part: &'a str,
}

// This function demonstrates lifetime elision rules
// Even though it takes a reference and returns a value derived from that reference,
// we don't need to specify lifetimes explicitly due to Rust's lifetime elision rules
pub fn calculate_length(s: &str) -> usize {
    s.len()
}

// This struct demonstrates lifetime bounds on generic types
// It holds two references that might have different lifetimes
pub struct Point<'a, 'b> {
    pub(crate) x: &'a i32,
    pub(crate) y: &'b i32,
}

// Implement a method for ImportantExcerpt
// This method doesn't need any additional lifetime annotations because it uses the lifetime from the impl block
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }
}

// This function demonstrates the 'static lifetime
// It returns a string slice with a 'static lifetime, which means it's available for the entire program run
#[allow(dead_code)]
pub fn static_lifetime() -> &'static str {
    "This string has a 'static lifetime"
}

// This struct demonstrates lifetime subtyping
// The 'announce lifetime must outlive the 'message and 'data lifetimes
pub struct Announcer<'message, 'data> {
    pub(crate) message: &'message str,
    pub(crate) data: &'data str,
}

// This trait demonstrates lifetime bounds on trait objects
pub trait PrintWithLifetime<'a> {
    fn print(&self);
}

// This struct implements the PrintWithLifetime trait
pub struct LifetimePrinter<'a> {
    pub(crate) s: &'a str,
}

impl<'a> PrintWithLifetime<'a> for LifetimePrinter<'a> {
    fn print(&self) {
        println!("Printing with lifetime: {}", self.s);
    }
}
