use std::collections::{HashMap, HashSet};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let at_i = nums.get(i).unwrap();
            let at_j = nums.get(j).unwrap();
            if at_i + at_j == target {
                result.push(i as i32);
                result.push(j as i32);
            }
            if result.len() == 2 {
                break;
            }
        }
        if result.len() == 2 {
            break;
        }
    }
    result
}

fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut enumerated_values = HashMap::<i32, usize>::new();
    let mut result = Vec::<i32>::new();
    for i in 0..nums.len() {
        let at_i = *nums.get(i).expect("Error at value at position {i}");
        let missing = target - at_i;
        if enumerated_values.contains_key(&missing) {
            let index = enumerated_values[&missing];
            result.push(i as i32);
            result.push(index as i32);
            break;
        }
        enumerated_values.insert(at_i, i);
    }
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![-3, 4, 3, 90], 0), vec![0, 2]);
    }

    #[test]
    fn test_two_sum2() {
        assert_eq!(two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum_2(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum_2(vec![-3, 4, 3, 90], 0), vec![0, 2]);
    }
}
