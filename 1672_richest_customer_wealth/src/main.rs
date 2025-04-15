pub fn maximum_wealth(accounts: &Vec<Vec<i32>>) -> i32 {
    let mut wealth = 0;
    for ac in accounts {
        let mut sum = 0;
        for val in ac {
            sum += val;
        }
        if sum > wealth {
            wealth = sum;
        }
    }
    wealth
}

#[cfg(test)]
mod test {
    use crate::maximum_wealth;

    #[test]
    fn test_ex1() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let result = maximum_wealth(&accounts);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_ex2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        let result = maximum_wealth(&accounts);
        assert_eq!(result, 10)
    }

    #[test]
    fn test_ex3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let result = maximum_wealth(&accounts);
        assert_eq!(result, 17)
    }
}

fn main() {
    println!("Hello, world!");
}
