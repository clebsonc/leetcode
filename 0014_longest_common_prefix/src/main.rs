use std::ops::Index;

pub fn common_prefix(strs: Vec<String>) -> String {
    let mut words = Vec::<Vec<char>>::new();
    for word in strs {
        let w = word.chars().collect::<Vec<char>>();
        words.push(w);
    }

    let first = &words[0];
    let mut prefix = String::new();
    for index in 0..first.len() {
        let mut valid_character = true;
        for word_index in 1..words.len() {
            let word = &words[word_index];

            if index == word.len() || word[index] != first[index] {
                valid_character = false;
                break;
            }
        }
        if valid_character == false {
            break;
        }
        prefix.push(first[index]);
    }
    prefix
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_common_prexix() {
        assert_eq!(
            common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string()
        );
        assert_eq!(
            common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ]),
            "".to_string()
        );
        assert_eq!(
            common_prefix(vec!["d".to_string(), "do".to_string(), "dog".to_string(),]),
            "d".to_string()
        );
        assert_eq!(
            common_prefix(vec!["ab".to_string(), "a".to_string()]),
            "a".to_string()
        );
    }
}

fn main() {
    println!("Hello, world!");
}
