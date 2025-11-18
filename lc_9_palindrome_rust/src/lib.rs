#![crate_type = "lib"]

pub fn is_palindrome(x: i32) -> bool {
    let as_string = x.to_string();
    let as_bytes = as_string.as_bytes();
    let length = as_bytes.len();

    let mid = length / 2;
    let last = length - 1;

    for i in 0..mid {
        if as_bytes[i] != as_bytes[last - i] {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(11));
        assert!(is_palindrome(0));
        assert!(!is_palindrome(-1));

        assert!(!is_palindrome(10));
        assert!(!is_palindrome(-121));
    }
}
