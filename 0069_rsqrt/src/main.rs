/// For this exercise we are not supposed to use any external function such as i32::sqrt.
pub fn my_sqrt(x: i32) -> i32 {
    // x.isqrt()

    let mut result: i32 = 0;
    for i in 0..=x as i64 {
        if (i * i) < x as i64 {
            continue;
        } else if (i * i) == x as i64 {
            result = i as i32;
        } else {
            result = (i - 1) as i32;
            break;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(2147395600), 46340);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
    }
}

fn main() {
    println!("{}", my_sqrt(2147395600));
}
