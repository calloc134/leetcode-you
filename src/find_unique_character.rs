pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // 出現したインデックスと回数を保存するためのハッシュマップ
        // キー→記号、値→(インデックス, 回数)のタプル
        let mut index_map = std::collections::HashMap::new();

        for (i, c) in s.chars().enumerate() {
            // キーが存在しない場合は、(i, 0)を挿入
            if !index_map.contains_key(&c) {
                index_map.insert(c, (i, 1));
            }
            // キーが存在する場合は、回数をインクリメント
            else {
                let (_, count) = index_map.get_mut(&c).unwrap();
                *count += 1;
            }
        }

        // 回数が1のものを洗い出して、最小のインデックスを返す
        let mut min_index = s.len();

        for (_, (i, count)) in index_map.iter() {
            if *count == 1 && *i < min_index {
                min_index = *i;
            }
        }

        // 回数が1のものがない場合は-1を返す
        if min_index == s.len() {
            return -1;
        }
        min_index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
