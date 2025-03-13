use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();

    let mut substring = HashSet::<char>::new();
    let mut max_substring_length: i32 = 0;
    let mut start_index: usize = 0;
    while start_index < chars.len() {
        if substring.len() > (chars.len() - start_index) {
            break;
        }
        for i in start_index..chars.len() {
            let c = chars[i];
            let found = substring.get(&c);
            match found {
                Some(_) => {
                    if substring.len() as i32 > max_substring_length {
                        max_substring_length = substring.len() as i32
                    };
                    substring.clear();
                    break;
                }
                None => {
                    substring.insert(c);
                }
            }
        }
        start_index = start_index + 1;
    }
    if substring.len() as i32 > max_substring_length {
        max_substring_length = substring.len() as i32
    };
    max_substring_length
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_substring() {
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3);
        assert_eq!(length_of_longest_substring(String::from("abcdef")), 6);
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}

fn main() {
    length_of_longest_substring(String::from("dvdf"));
    println!("Hello, world!");
}
