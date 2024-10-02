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

        // 右のインデックスが最後を超えるまでループ
        'outer: while right < s.len() {
            // 含まれている文字の集合
            // ハッシュマップじゃなくて、すでに含まれているかどうかを判定するため集合を使う
            // この集合に存在していれば既知のアルファベットや！
            let mut count_map = std::collections::HashSet::new();

            println!("left: {}, right: {}", left, right);

            // 含まれている文字を追加、もしくはカウントを増やす
            for c in s[left..=right].iter() {
                println!("c: {}", c);
                if count_map.contains(c) {
                    // 重複がある場合、左端を進める
                    // その後whileループの方でcontimue
                    left += 1;
                    continue 'outer;
                    // 汚いコードでごめんなさいの顔
                } else {
                    count_map.insert(c);
                }
            }

            // 重複がない場合
            // もし最大長よりも長い場合、最大長を更新
            if max_length < right - left + 1 {
                max_length = right - left + 1;
            }

            // 右端を進める
            right += 1;
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
