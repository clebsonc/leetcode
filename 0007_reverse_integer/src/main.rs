pub fn reverse(x: i32) -> i32 {
    let mut s = x.to_string();
    let mut v = Vec::<char>::new();
    let mut is_negative = false;
    while s.len() > 0 {
        let c = s.pop().unwrap();
        if c == '-' {
            is_negative = true;
        } else {
            v.push(c);
        }
    }
    let result = String::from_iter(v).parse::<i32>();
    let mut res = match result {
        Ok(val) => val,
        _ => 0,
    };
    if is_negative {
        res *= -1;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
    }
}

fn main() {
    println!("Hello, world!");
}
