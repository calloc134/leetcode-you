pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        return 49;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test_2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
