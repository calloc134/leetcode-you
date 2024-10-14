pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
