pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // get index position
    let mut index = 0;
    let mut size = nums.len();
    while index < size {
        let val_at_index = nums[index];
        if val_at_index == val {
            nums.swap_remove(index);
            size = size - 1;
        } else {
            index = index + 1;
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_remove_element_0() {
        let mut nums = Vec::from([3, 2, 2, 3]);
        let result = remove_element(&mut nums, 3);
        println!("{:#?}", nums);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_remove_element_1() {
        let mut nums = Vec::from([2, 2, 2, 2]);
        let result = remove_element(&mut nums, 2);
        println!("{:#?}", nums);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_remove_element_3() {
        let mut nums = Vec::from([3, 2, 1, 0]);
        let result = remove_element(&mut nums, 4);
        println!("{:#?}", nums);

        assert_eq!(result, 4);
    }
}

fn main() {
    println!("Hello, world!");
}
