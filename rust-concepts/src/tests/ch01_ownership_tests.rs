use crate::ch01_ownership;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch01_ownership_demo() {
        // This test will call the demo function and ensure it doesn't panic
        ch01_ownership::demo();
    }

    #[test]
    fn test_gives_ownership() {
        let s = ch01_ownership::gives_ownership();
        assert_eq!(s, "yours");
    }

    #[test]
    fn test_takes_and_gives_back() {
        let original = String::from("hello");
        let returned = ch01_ownership::takes_and_gives_back(original);
        assert_eq!(returned, "hello");
    }

    #[test]
    fn test_move_semantics() {
        let s1 = String::from("hello");
        let s2 = s1;
        // Uncommenting the next line would cause a compile error
        // assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_copy_trait() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }
}
