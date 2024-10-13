pub struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        // 制約が1~3999なので4000以降は考えなくて良い
        struct RomanSource {
            thousand: usize,
            hundred: usize,
            ten: usize,
            one: usize,
        }

        // まずは1000, 100, 10, 1の位を取得
        let roman_source = RomanSource {
            thousand: (num / 1000) as usize,
            hundred: ((num % 1000) / 100) as usize,
            ten: ((num % 100) / 10) as usize,
            one: (num % 10) as usize,
        };

        // マッチ式で処理
        fn roman_num(
            num: usize,
            alphabet_one: char,
            alphabet_five: char,
            alphabet_ten: char,
        ) -> String {
            match num {
                0 => "".to_string(),
                1..=3 => alphabet_one.to_string().repeat(num),
                4 => format!("{}{}", alphabet_one, alphabet_five),
                5 => alphabet_five.to_string(),
                6..=8 => format!(
                    "{}{}",
                    alphabet_five,
                    alphabet_one.to_string().repeat(num - 5)
                ),
                9 => format!("{}{}", alphabet_one, alphabet_ten),
                _ => "".to_string(),
            }
        }

        // それぞれの位をローマ数字に変換
        let thousand = roman_num(roman_source.thousand, 'M', ' ', ' ');
        let hundred = roman_num(roman_source.hundred, 'C', 'D', 'M');
        let ten = roman_num(roman_source.ten, 'X', 'L', 'C');
        let one = roman_num(roman_source.one, 'I', 'V', 'X');

        // 連結して返す
        format!("{}{}{}{}", thousand, hundred, ten, one)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 3749;
        assert_eq!(Solution::int_to_roman(num), "MMMDCCXLIX".to_string());
    }
    #[test]
    fn test_2() {
        let num = 58;
        assert_eq!(Solution::int_to_roman(num), "LVIII".to_string());
    }
    #[test]
    fn test_3() {
        let num = 1994;
        assert_eq!(Solution::int_to_roman(num), "MCMXCIV".to_string());
    }
}
