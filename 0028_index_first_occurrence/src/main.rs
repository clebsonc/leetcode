pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(val) = haystack.find(&needle) {
        return val as i32;
    }
    -1
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::*;

    #[test]
    fn test_str_str() {
        assert_eq!(str_str(String::from("sadbustsad"), String::from("sad")), 0);
        assert_eq!(str_str(String::from("leetcode"), String::from("leeto")), -1);
    }
}

fn main() {
    println!("Hello, world!");
}
