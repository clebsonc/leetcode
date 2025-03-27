fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut running_sum = 0;
    for i in nums {
        running_sum = running_sum + i;
        result.push(running_sum);
    }
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(vec![1, 3, 6, 10], running_sum(vec![1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4, 5], running_sum(vec![1, 1, 1, 1, 1]));
    }
}

fn main() {
    println!("Hello, world!");
}
