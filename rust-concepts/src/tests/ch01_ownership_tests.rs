use crate::ch01_ownership;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch01_ownership_demo() {
        // This test calls the demo function from ch01_ownership module
        // It doesn't assert anything specific, but ensures the function runs without panicking
        // This is useful for demonstrating that the demo code compiles and executes successfully
        ch01_ownership::demo();
    }

    #[test]
    fn test_gives_ownership() {
        // This test checks the gives_ownership function from ch01_ownership module
        // It verifies that the function returns a String with the expected value
        let s = ch01_ownership::gives_ownership();
        assert_eq!(s, "yours");
        // This test also implicitly checks that we now own the returned String
        // and can use it freely without borrowing issues
    }

    #[test]
    fn test_takes_and_gives_back() {
        // This test verifies the behavior of the takes_and_gives_back function
        // It creates a String, passes ownership to the function, and checks the returned value
        let original = String::from("hello");
        let returned = ch01_ownership::takes_and_gives_back(original);
        assert_eq!(returned, "hello");
        // This test demonstrates that ownership is transferred to the function and then back to the caller
        // Note that we can't use 'original' here as its ownership was moved to the function
    }

    #[test]
    fn test_move_semantics() {
        // This test demonstrates Rust's move semantics for types that don't implement Copy
        let s1 = String::from("hello");
        let s2 = s1;
        // Uncommenting the next line would cause a compile error because s1's value was moved to s2
        // assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
        // This test shows that after moving s1 to s2, we can only use s2
        // It highlights Rust's ownership rules preventing use of moved values
    }

    #[test]
    fn test_copy_trait() {
        // This test demonstrates the behavior of types that implement the Copy trait
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
        // Unlike the String in the previous test, integers implement Copy
        // This means x is copied to y, and both can be used independently
        // This test shows the difference in behavior between Copy and non-Copy types
    }
}
