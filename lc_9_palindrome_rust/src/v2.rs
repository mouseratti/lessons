pub fn is_palindrome_v2(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let as_string: String = x.to_string();
    let reversed: String = as_string.chars().rev().collect();
    reversed == as_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_palindrome_v2(121));
        assert!(is_palindrome_v2(11));
        assert!(is_palindrome_v2(0));
        assert!(!is_palindrome_v2(-1));
        assert!(!is_palindrome_v2(10));
        assert!(!is_palindrome_v2(-121));
    }
}
