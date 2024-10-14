pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_char = s.chars().collect::<Vec<char>>();
        let t_char = t.chars().collect::<Vec<char>>();

        // まず左と右のポインタを用意
        // 左
        let mut s_left_cursor = 0;
        // let mut s_right_cursor = s.len() - 1;
        let mut t_left_cursor = 0;
        // let mut t_right_cursor = t.len() - 1;

        if s_char.len() == 0 {
            return true;
        }

        // それぞれの文字列の左から順に比較していく

        while s_left_cursor < s_char.len() && t_left_cursor < t_char.len() {
            if s_char[s_left_cursor] == t_char[t_left_cursor] {
                s_left_cursor += 1;
            }
            t_left_cursor += 1;
        }

        if s_left_cursor == s_char.len() {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }
}
