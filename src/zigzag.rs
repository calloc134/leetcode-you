pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // もしnum_rowsが1以下の場合はそのまま返す
        if num_rows <= 1 {
            return s;
        }

        // modを行う数字について求める
        let mod_number = (num_rows - 1) * 2;

        // 与えられた文字列について
        // enumerateを使ってインデックスを含んだタプルにしながらcharベクタに変換
        let s_char_with_index = s.chars().enumerate().collect::<Vec<(usize, char)>>();

        // 行数分のベクタを用意
        let mut rows = vec![String::new(); num_rows as usize];

        // 与えられた文字列についてループ
        for (i, c) in s_char_with_index {
            // mod_numberで割った余りを求める
            let remainder = i % mod_number as usize;

            // まずあまりが中央より小さい場合
            if remainder < num_rows as usize {
                // そのまま行に追加
                rows[remainder].push(c);
            } else {
                // あまりが中央より大きい場合
                // 中央からの差分を求める
                let diff = remainder - num_rows as usize;

                // 中央からの差分を引いた行に追加
                rows[num_rows as usize - 2 - diff].push(c);
            }
        }

        // 行数分の文字列を結合して返す
        rows.join("")
    }
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
