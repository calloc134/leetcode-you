pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_palindrome1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bab".to_string());
    }
    #[test]
    fn test_longest_palindrome2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb".to_string());
    }
    #[test]
    fn test_longest_palindrome3() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a".to_string());
    }
    #[test]
    fn test_longest_palindrome4() {
        let s = "ac".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a".to_string());
    }
    #[test]
    fn test_longest_palindrome5() {
        let s = "bb".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb".to_string());
    }
}
