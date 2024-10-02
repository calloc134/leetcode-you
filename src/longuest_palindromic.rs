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

        // もしかしてシャクトリ法じゃなくて組み合わせでは？
        // 数字の組み合わせのタプルを取得
        let collaboration_tuple = (0..s.len())
            .flat_map(|i| (i..s.len()).map(move |j| (i, j)))
            .collect::<Vec<(usize, usize)>>();

        // println!("{:?}", collaboration_tuple);

        // 回文かどうかを判定する関数
        fn is_palindrome(s: &Vec<char>, i: usize, j: usize) -> bool {
            let mut i = i;
            let mut j = j;
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            return true;
        }

        // この組み合わせに沿って、回文になっているかをすべて調べる
        for (i, j) in collaboration_tuple {
            // もし回文になっているなら、最長の長さを更新する
            if is_palindrome(&s, i, j) {
                let length = j - i + 1;
                if length > result.max_length {
                    result.max_length = length;
                    result.string = s[i..=j].iter().collect();
                }
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
