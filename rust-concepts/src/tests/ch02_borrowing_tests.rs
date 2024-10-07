use crate::ch02_borrowing;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch02_borrowing_demo() {
        // This test will call the demo function and ensure it doesn't panic
        ch02_borrowing::demo();
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(ch02_borrowing::calculate_length(&s), 5);
    }

    #[test]
    fn test_change() {
        let mut s = String::from("hello");
        ch02_borrowing::change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_no_dangle() {
        let result = ch02_borrowing::no_dangle();
        assert_eq!(result, "hello");
    }
}
