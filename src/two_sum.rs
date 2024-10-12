pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // ハッシュマップを作成
        // キー→数値、値→インデックス
        let mut hash_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            // ターゲットから数値を引いた値がハッシュマップに存在するか
            let diff = target - num;
            if let Some(&index) = hash_map.get(&diff) {
                return vec![index, i as i32];
            }
            // 存在しない場合はハッシュマップに数値とインデックスを追加
            hash_map.insert(num.clone(), i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_4() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 3]);
    }
}
