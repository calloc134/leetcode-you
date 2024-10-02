pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 問題の意図まとめ
        // 登場する文字に被りがないようにしないといけない
        // これってシャクトリ法？というので解けるんじゃないか？

        // Stringをcharに変換
        let s: Vec<char> = s.chars().collect();

        // ここから効率化のためにわかりきっている結果を返す部分

        // もし空文字なら、0を返す
        if s.len() == 0 {
            return 0;
        }

        // もし一文字以下なら、1を返す
        if s.len() == 1 {
            return 1;
        }

        // ここまで

        // まず、左端と右端のインデックスを格納する変数定義
        let mut left = 0;
        let mut right = 0;

        // 最長の長さを格納する変数
        // 最終的にこれが結果になりますやんか
        let mut max_length = 0;

        // 含まれている文字の集合
        // ハッシュマップじゃなくて、すでに含まれているかどうかを判定するため集合を使う
        // すべてのループで共有することができる
        // この集合に存在していれば既知のアルファベットや！
        let mut count_map = std::collections::HashSet::new();

        // 右のインデックスが最後を超えるまでループ
        while right < s.len() {
            // もしcount_mapに含まれているなら
            if count_map.contains(&s[right]) {
                // 左端を進める
                count_map.remove(&s[left]);
                left += 1;
            } else {
                // もし含まれていないなら、右端を進める
                count_map.insert(s[right]);
                // 最長の長さを更新
                max_length = std::cmp::max(max_length, right - left + 1);
                right += 1;
            }
        }

        return max_length as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test2() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn test4() {
        let s = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 0);
    }

    #[test]
    fn test5() {
        let s = " ".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test5alpha() {
        let s = "a".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn test6() {
        let s = "au".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 2);
    }
}
