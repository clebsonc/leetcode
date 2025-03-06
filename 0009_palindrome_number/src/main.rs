/// Return if a given number is or not a palindrome
///
///  https://leetcode.com/problems/palindrome-number/description/
fn is_palindrome(value: i32) -> bool {
    let chars: Vec<char> = value.to_string().chars().collect();
    for i in 0..chars.len() {
        let j = chars.len() - i - 1;
        if chars[i] != chars[j] {
            return false;
        }
        if j < i {
            break;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(123321), true);
    }
}

fn main() {
    println!("Hello, world!");
}
