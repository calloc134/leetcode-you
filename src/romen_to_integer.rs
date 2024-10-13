pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let s = "III".to_string();
        assert_eq!(Solution::roman_to_int(s), 3);
    }
    #[test]
    fn test_roman_to_int_2() {
        let s = "LVIII".to_string();
        assert_eq!(Solution::roman_to_int(s), 58);
    }

    #[test]
    fn test_roman_to_int_3() {
        let s = "MCMXCIV".to_string();
        assert_eq!(Solution::roman_to_int(s), 1994);
    }
}
