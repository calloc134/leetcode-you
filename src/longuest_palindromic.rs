pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // charのベクタに変換
        let s: Vec<char> = s.chars().collect();

        // もし空文字なら空文字を返す
        if s.len() == 0 {
            return "".to_string();
        }

        // もし一文字ならその文字を返す
        if s.len() == 1 {
            return s[0].to_string();
        }

        // 回文で最長の長さを保持する変数
        struct Result {
            max_length: usize,
            string: String,
        }

        let mut result = Result {
            max_length: 0,
            string: "".to_string(),
        };

        // 両端のインデックスを定義
        let mut left = 0;
        let mut right = 0;

        // シャクトリする外側ループ
        'outer: while right < s.len() {
            // 回文であるか検証する内側ループ
            let mut buffer = vec![];
            for i in left..=right {
                buffer.push(s[i]);
            }

            if buffer == buffer.iter().rev().cloned().collect::<Vec<char>>() {
                // 回文であれば、次の文字を検証する
                // まず、
                if right - left + 1 > result.max_length {
                    result = Result {
                        max_length: right - left + 1,
                        string: buffer.iter().collect(),
                    };
                }
                right += 1;
            } else {
                // 回文でなければ、左端を次に進める
                left += 1;
            }
        }

        return result.string;
    }
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
