pub struct Solution;

impl Solution {
    fn does_overflow(s_char: &Vec<i32>, is_negative: bool) -> bool {
        if s_char.len() > 10 {
            return true;
        }

        if s_char.len() < 10 {
            return false;
        }

        let max = if is_negative {
            vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 8]
        } else {
            vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7]
        };

        for i in 0..s_char.len() {
            if s_char[i] > max[i] {
                return true;
            } else if s_char[i] < max[i] {
                return false;
            }
        }

        return false;
    }

    pub fn my_atoi(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        // 先頭と末尾の空白を削除
        let mut s_char_without_space = s.trim().chars().collect::<Vec<char>>();
        if s_char_without_space.len() == 0 {
            return 0;
        }

        // マイナスになるかどうか
        let is_negative: Option<bool> = match s_char_without_space[0] {
            '+' => Some(false),
            '-' => Some(true),
            _ => None,
        };
        // 符号の場合は削除
        if is_negative.is_some() {
            s_char_without_space.remove(0);
        }

        // 数字の入るキュー
        let mut result_queue: Vec<i32> = Vec::new();

        // 文字の終わりを判定したいためwhileで
        for i in 0..s_char_without_space.len() {
            let result = s_char_without_space[i].to_digit(10);
            if result.is_none() {
                break;
            }

            if result.unwrap() != 0 || result_queue.len() != 0 {
                result_queue.push(result.unwrap() as i32);
            }
        }

        // オーバーフロー判定
        if Solution::does_overflow(&result_queue, is_negative.unwrap_or(false)) {
            return if is_negative.unwrap_or(false) {
                i32::MIN
            } else {
                i32::MAX
            };
        }

        // キューの内容を数値に復元
        let mut result_num = 0;
        for i in 0..result_queue.len() {
            result_num = result_num * 10 + result_queue[i];
        }

        // フラグからマイナスをつけて返す
        return result_num * if is_negative.unwrap_or(false) { -1 } else { 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi1() {
        let s = "42".to_string();
        assert_eq!(Solution::my_atoi(s), 42);
    }

    #[test]
    fn test_my_atoi2() {
        let s = " -042".to_string();
        assert_eq!(Solution::my_atoi(s), -42);
    }

    #[test]
    fn test_my_atoi3() {
        let s = "1337c0d3".to_string();
        assert_eq!(Solution::my_atoi(s), 1337);
    }

    #[test]
    fn test_my_atoi4() {
        let s = "0-1".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_my_atoi5() {
        let s = "words and 987".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_my_atoi6() {
        let s = "-91283472332".to_string();
        assert_eq!(Solution::my_atoi(s), -2147483648);
    }

    #[test]
    fn test_my_atoi7() {
        let s = "21474836460".to_string();
        assert_eq!(Solution::my_atoi(s), 2147483647);
    }

    #[test]
    fn test_my_atoi8() {
        let s = " ".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_my_atoi9() {
        let s = "   +0 123".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_my_atoi10() {
        let s = "2147483648".to_string();
        assert_eq!(Solution::my_atoi(s), 2147483647);
    }

    #[test]
    fn test_my_atoi11() {
        let s = "2147483800".to_string();
        assert_eq!(Solution::my_atoi(s), 2147483647);
    }
}
