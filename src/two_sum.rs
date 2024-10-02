pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // インデックスをつけてタプルにする
        // index, valueの順番にする
        let nums_with_index: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();

        // target以上の値を持つ要素を取り除く
        let nums_with_index: Vec<(usize, i32)> = nums_with_index
            .into_iter()
            .filter(|(_, num)| *num < target)
            .collect();

        // target - xにした

        // target - xにした配列を作る
        // let target_minus_x: Vec<(usize, i32)> = nums_with_index
        //     .iter()
        //     .map(|(index, num)| (*index, target - num))
        //     .collect();

        // // target - xにした配列と元の配列を比較して、一致するものを返却
        // for (index, num) in target_minus_x {
        //     for (i, n) in nums_with_index.iter() {
        //         if num == *n && index != *i {
        //             return vec![index as i32, *i as i32];
        //         }
        //     }
        // }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
