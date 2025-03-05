fn solution1(value: String) -> usize {
    let mut chars = value.chars();

    let mut prev = chars.next().unwrap() as usize;

    let mut curr_difference = Vec::<usize>::new();
    if value.len() > 1 {
        for c in chars {
            let current = c as usize;
            let difference = prev.abs_diff(current);
            curr_difference.push(difference);
            prev = current;
        }

        let mut result = 0;
        for value in curr_difference {
            result = result + value;
        }
        return result;
    }
    prev
}

fn solution2(value: String) -> u32 {
    let mut sum = 0;
    let chars = value.chars().collect::<Vec<char>>();

    if chars.len() == 1 {
        return *chars.get(0).unwrap() as u32;
    }
    for i in 0..chars.len() - 1 {
        let left = *chars.get(i).unwrap() as i32;
        let right = *chars.get(i + 1).unwrap() as i32;
        sum += left.abs_diff(right);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{solution1, solution2};

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(String::from("hello")), 13);
        assert_eq!(solution1(String::from("zaz")), 50);
        assert_eq!(solution1(String::from("a")), 97);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(String::from("hello")), 13);
        assert_eq!(solution2(String::from("zaz")), 50);
        assert_eq!(solution2(String::from("a")), 97);
    }
}

fn main() {
    println!("hi");
}
