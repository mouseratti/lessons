pub fn is_palindrome_v3(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let mut tmp: i64 = x;
    let mut stor: i64 = 0;
    while tmp != 0 {
        stor = stor * 10 + tmp % 10;
        tmp /= 10;
    }
    stor == x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_palindrome_v3(121));
        assert!(is_palindrome_v3(11));
        assert!(is_palindrome_v3(0));
        assert!(is_palindrome_v3(123456789987654321));
        assert!(!is_palindrome_v3(-1));
        assert!(!is_palindrome_v3(10));
        assert!(!is_palindrome_v3(-121));
        assert!(!is_palindrome_v3(123456788987654321));

        
    }
}
