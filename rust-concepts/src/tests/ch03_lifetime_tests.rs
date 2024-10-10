use crate::ch03_lifetime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ch03_lifetime_demo() {
        // This test will call the demo function and ensure it doesn't panic
        ch03_lifetime::demo();
    }

    #[test]
    fn test_longest() {
        let string1 = String::from("short");
        let string2 = String::from("longer");
        assert_eq!(ch03_lifetime::longest(&string1, &string2), "longer");
    }

    #[test]
    fn test_important_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ch03_lifetime::ImportantExcerpt {
            part: first_sentence,
        };
        assert_eq!(i.part, "Call me Ishmael");
        assert_eq!(i.level(), 3);
    }

    #[test]
    fn test_static_lifetime() {
        assert_eq!(ch03_lifetime::static_lifetime(), "This string has a 'static lifetime");
    }
}