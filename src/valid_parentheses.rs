pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let bracket_hashset = std::collections::HashSet::from(['(', ')', '[', ']', '{', '}']);

        let bracket_start_hashset = std::collections::HashSet::from(['(', '[', '{']);
        let bracket_end_hashset = std::collections::HashSet::from([')', ']', '}']);

        let bracket_pair_hashmap =
            std::collections::HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

        let mut stack: Vec<char> = Vec::new();

        let s_char = s.chars().collect::<Vec<char>>();

        for char in s_char {
            if !bracket_hashset.contains(&char) {
                continue;
            }

            if bracket_end_hashset.contains(&char) {
                if let Some(last) = stack.pop() {
                    if last != char {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            if bracket_start_hashset.contains(&char) {
                stack.push(*bracket_pair_hashmap.get(&char).unwrap());
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }

    #[test]
    fn test_6() {
        assert_eq!(Solution::is_valid("[".to_string()), false);
    }

    #[test]
    fn test_7() {
        assert_eq!(Solution::is_valid("]".to_string()), false);
    }
}
