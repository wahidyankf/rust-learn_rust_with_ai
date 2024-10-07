use crate::ch01_ownership;
use crate::ch02_borrowing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch01_ownership() {
        // This test will call the demo function and ensure it doesn't panic
        ch01_ownership::demo();
    }

    #[test]
    fn test_ch02_borrowing() {
        // This test will call the demo function and ensure it doesn't panic
        ch02_borrowing::demo();
    }

    // Add more specific tests here as needed
}
