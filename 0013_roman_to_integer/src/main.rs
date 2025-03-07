/// https://leetcode.com/problems/roman-to-integer/
mod romans;

#[cfg(test)]
mod test {
    use crate::romans::*;
    #[test]
    fn test_roman_to_integer() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
        assert_eq!(roman_to_int(String::from("MDCCCLXXX")), 1880);
        assert_eq!(roman_to_int(String::from("DCXXI")), 621);
        assert_eq!(roman_to_int(String::from("XCIX")), 99);
        assert_eq!(roman_to_int(String::from("III")), 3);
        assert_eq!(roman_to_int(String::from("IV")), 4);
        assert_eq!(roman_to_int(String::from("IX")), 9);
        assert_eq!(roman_to_int(String::from("X")), 10);
        assert_eq!(roman_to_int(String::from("XXX")), 30);
        assert_eq!(roman_to_int(String::from("CCC")), 300);
        assert_eq!(roman_to_int(String::from("C")), 100);
        assert_eq!(roman_to_int(String::from("I")), 1);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("M")), 1000);
        assert_eq!(roman_to_int(String::from("XCIV")), 94);
        assert_eq!(roman_to_int(String::from("XCVIII")), 98);
    }
}

fn main() {
    println!("Hello, world!");
}
