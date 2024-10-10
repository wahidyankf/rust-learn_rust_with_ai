use crate::ch03_lifetime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch03_lifetime_demo() {
        // This test calls the demo function from the ch03_lifetime module
        // It doesn't assert anything specific, but ensures the function runs without panicking
        // This is useful for demonstrating that all the lifetime examples in the demo compile and execute successfully
        ch03_lifetime::demo();
    }

    #[test]
    fn test_longest() {
        // This test verifies the behavior of the 'longest' function, which demonstrates basic lifetime annotations
        let string1 = String::from("short");
        let string2 = String::from("longer");
        // The 'longest' function should return a reference to the longer of the two strings
        // This test also implicitly checks that the lifetime annotations in 'longest' are correct,
        // as the returned reference must be valid for the lifetime of both input references
        assert_eq!(ch03_lifetime::longest(&string1, &string2), "longer");
    }

    #[test]
    fn test_important_excerpt() {
        // This test checks the 'ImportantExcerpt' struct, which demonstrates lifetimes in struct definitions
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ch03_lifetime::ImportantExcerpt {
            part: first_sentence,
        };
        // Verify that the 'part' field of ImportantExcerpt contains the expected string
        assert_eq!(i.part, "Call me Ishmael");
        // Test the 'level' method, which doesn't require additional lifetime annotations
        assert_eq!(i.level(), 3);
        // This test ensures that the lifetime annotations in ImportantExcerpt allow it to hold a reference correctly
    }

    #[test]
    fn test_static_lifetime() {
        // This test verifies the 'static_lifetime' function, which demonstrates the 'static lifetime
        // The function should return a string slice with a 'static lifetime
        assert_eq!(
            ch03_lifetime::static_lifetime(),
            "This string has a 'static lifetime"
        );
        // This test shows that 'static lifetime values are available for the entire program run
    }

    #[test]
    fn test_calculate_length() {
        // This test checks the 'calculate_length' function, which demonstrates lifetime elision
        let s = String::from("hello");
        // The function should return the length of the string slice without needing explicit lifetime annotations
        assert_eq!(ch03_lifetime::calculate_length(&s), 5);
        // This test implicitly verifies that Rust's lifetime elision rules work correctly in this case
    }

    #[test]
    fn test_point() {
        // This test verifies the 'Point' struct, which demonstrates lifetime bounds on generic types
        let x = 5;
        let y = 10;
        let point = ch03_lifetime::Point { x: &x, y: &y };
        // Check that the Point struct correctly stores and allows access to the references
        assert_eq!(*point.x, 5);
        assert_eq!(*point.y, 10);
        // This test ensures that the lifetime annotations in Point allow it to hold references with different lifetimes
    }

    #[test]
    fn test_announcer() {
        // This test checks the 'Announcer' struct, which demonstrates lifetime subtyping
        let s1 = String::from("Hello");
        let s2 = String::from("World");
        let announce = ch03_lifetime::Announcer {
            message: &s1,
            data: &s2,
        };
        // Verify that the Announcer struct correctly stores and allows access to the references
        assert_eq!(announce.message, "Hello");
        assert_eq!(announce.data, "World");
        // This test ensures that the lifetime subtyping in Announcer works correctly,
        // allowing references with different lifetimes to be stored in the same struct
    }

    #[test]
    fn test_lifetime_printer() {
        // This test verifies the 'LifetimePrinter' struct and 'PrintWithLifetime' trait,
        // which demonstrate lifetime bounds on trait objects
        let s = String::from("Test");
        let printer = ch03_lifetime::LifetimePrinter { s: &s };
        let obj: Box<dyn ch03_lifetime::PrintWithLifetime> = Box::new(printer);
        // Call the print method to ensure it doesn't panic
        // This test doesn't assert any specific output, but checks that the method can be called
        // on a trait object with a lifetime bound
        obj.print();
        // This test ensures that the lifetime annotations on the trait and struct work correctly together,
        // allowing for dynamic dispatch while maintaining lifetime safety
    }
}
