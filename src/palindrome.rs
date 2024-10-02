struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // 当然マイナスは回文にならない
        if x < 0 {
            return false;
        }

        // いきなり最上位はわからない
        // だるま落としみたいに
        // 最下位を引っこ抜いて別の変数に詰めていき、ひっくり返した変数を作る

        let mut reversed_memory = 0;
        let mut source = x;

        while (source > 0) {
            // 逆メモリを10倍(左シフト)する
            reversed_memory *= 10;
            // ソースの最下位の数字を取り出す
            let last_digit = source % 10;
            // 逆メモリに最下位の数字を詰める
            reversed_memory += last_digit;
            // ソースを10で割り、右シフト
            source = source / 10;
        }

        // 逆メモリとソースが一致していれば回文
        return x == reversed_memory;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
