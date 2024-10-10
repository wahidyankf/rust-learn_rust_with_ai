use crate::ch02_borrowing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch02_borrowing_demo() {
        // This test calls the demo function from ch02_borrowing module
        // It doesn't assert anything specific, but ensures the function runs without panicking
        // This is useful for demonstrating that the demo code compiles and executes successfully
        ch02_borrowing::demo();
    }

    #[test]
    fn test_calculate_length() {
        // This test checks the calculate_length function from ch02_borrowing module
        // It creates a String, passes a reference to it to calculate_length,
        // and asserts that the returned length is correct
        let s = String::from("hello");
        assert_eq!(ch02_borrowing::calculate_length(&s), 5);
        // This test also implicitly checks that calculate_length doesn't take ownership of the string,
        // as we can still use 's' after calling the function
    }

    #[test]
    fn test_change() {
        // This test verifies the behavior of the change function from ch02_borrowing module
        // It creates a mutable String, passes a mutable reference to change,
        // and checks if the string was modified as expected
        let mut s = String::from("hello");
        ch02_borrowing::change(&mut s);
        assert_eq!(s, "hello, world");
        // This test demonstrates mutable borrowing, showing that change can modify the original string
    }

    #[test]
    fn test_no_dangle() {
        // This test checks the no_dangle function from ch02_borrowing module
        // It calls no_dangle and verifies the returned string
        let result = ch02_borrowing::no_dangle();
        assert_eq!(result, "hello");
        // This test ensures that no_dangle returns a valid String without any dangling references
        // It's particularly important as it demonstrates Rust's ability to prevent dangling pointers
    }
}
