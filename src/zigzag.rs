pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = "PAHNAPLSIIGYIR";
        assert_eq!(Solution::convert(s, num_rows), result);
    }

    #[test]
    fn test_2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = "PINALSIGYAHRPI";
        assert_eq!(Solution::convert(s, num_rows), result);
    }

    #[test]
    fn test_3() {
        let s = "A".to_string();
        let num_rows = 1;
        let result = "A";
        assert_eq!(Solution::convert(s, num_rows), result);
    }
}
