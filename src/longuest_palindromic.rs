pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // まずは前処理を行う
        // 先頭と末尾、文字の間に区切り文字 '#' を挿入する
        // これにより奇数長と偶数長の回文を同様に扱えるようになる
        let mut processed_string: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
        // まずは先頭に '#' を挿入
        processed_string.push('#');
        // 文字列 's' の各文字の間に '#' を挿入
        for c in s.chars() {
            processed_string.push(c);
            processed_string.push('#');
        }

        // 前処理後の文字列の長さ
        let length = processed_string.len();

        // 各文字位置での回文の半径を格納する配列
        let mut palindrome_radii = vec![0; length];

        // 現在の回文の右端
        // この右端はループにしたがって更新されていく
        let mut right = 0;

        // 文章を一文字ずつ処理していく
        for i in 0..length {
            // 回文を左右に拡張していく処理
            // 中心を 'i' とする

            // 3つの条件
            // 1. 現在のインデックス+回文の半径+1が文字列の長さを超えない(回文の右端が文字列の右端を超えない)
            // 2. 現在のインデックスが回文の半径より大きい(回文の左端が文字列の左端を超えない)
            // 3. 回文の左端と右端が等しい
            // これらの条件を満たす限り、回文の半径を拡張していく

            // nを半径とし、i+n+1文字目とi-n-1文字目が等しい場合、回文の半径をn+1に更新する繰り返し
            while i + palindrome_radii[i] + 1 < length
                && i >= palindrome_radii[i] + 1
                && processed_string[i + palindrome_radii[i] + 1]
                    == processed_string[i - palindrome_radii[i] - 1]
            {
                palindrome_radii[i] += 1;
            }

            // 回文が右端を超えた場合、右端を更新
            if i + palindrome_radii[i] > right {
                right = i + palindrome_radii[i];
            }
        }

        // 得られた最長回文長のベクタをもとに、該当する文字列を求める処理
        // まず最大の回文長を持つ中心を求める
        let max_center = palindrome_radii.iter().enumerate().fold(0, |max, (i, &x)| {
            if x > palindrome_radii[max] {
                i
            } else {
                max
            }
        });

        // 長さも求める
        let max_len = palindrome_radii[max_center];

        // 回文の開始位置と終了位置を計算
        // 中心については分かっているので
        let start = max_center - max_len;
        let end = max_center + max_len;

        // 回文部分文字列を構築（区切り文字 '#' を除去）
        let palindrome = processed_string[start..=end]
            .iter()
            .filter(|&&c| c != '#')
            .collect();

        // 最長回文部分文字列を返す
        palindrome
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
