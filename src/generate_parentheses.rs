pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

        let combination = generate_combinations(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![["((()))", "(()())", "(())()", "()(())", "()()()"]]
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::generate_parenthesis(1), vec![["()"]])
    }
}
