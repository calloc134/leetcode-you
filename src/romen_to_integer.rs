use std::collections::HashMap;


pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_char = s.chars().collect::<Vec<char>>();

        // ローマ字の対応表のハッシュマップを作成
        let roman_map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let s_roman = s_char
            .iter()
            .map(|&c| roman_map.get(&c).unwrap())
            .collect::<Vec<&i32>>();

        let mut result = 0;

        // 末尾から見ていく
        for count in (0..s_roman.len()).rev() {
            // もし最初でなく、現在の数字が次の数字より小さい場合は、引く
            // if s_roman[count] < s_roman[count + 1] {
            if count < s_roman.len() - 1 && s_roman[count] < s_roman[count + 1] {
                result -= s_roman[count];
            } else {
                result += s_roman[count];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let s = "III".to_string();
        assert_eq!(Solution::roman_to_int(s), 3);
    }
    #[test]
    fn test_roman_to_int_2() {
        let s = "LVIII".to_string();
        assert_eq!(Solution::roman_to_int(s), 58);
    }

    #[test]
    fn test_roman_to_int_3() {
        let s = "MCMXCIV".to_string();
        assert_eq!(Solution::roman_to_int(s), 1994);
    }
}
