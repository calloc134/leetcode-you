pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // leftは処理済み要素の右隣
        let mut left_pointer = 0;
        // rightは探索中の要素、0かどうかは不明、これから調査する
        let mut right_pointer = 1;

        while right_pointer < nums.len() && left_pointer < nums.len() {
            // パターンマッチで判断
            match (nums[left_pointer], nums[right_pointer]) {
                // leftが0でrightが0の場合、rightを進める
                (0, 0) => {
                    right_pointer += 1;
                }
                // leftが0でrightが非0の場合、swapしてleftとrightを進める
                (0, _) => {
                    nums.swap(left_pointer, right_pointer);
                    left_pointer += 1;
                    right_pointer += 1;
                }
                // leftが非0でrightが0の場合、leftとrightを進める
                // (_, _)に統一できるので省略
                // (_, 0) => {
                    // left_pointer += 1;
                    // right_pointer += 1;
                // }
                // leftが非0でrightが非0の場合、leftを進める
                (_, _) => {
                    left_pointer += 1;
                    right_pointer += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_example_3() {
        let mut nums = vec![1, 0, 1];
        let expected = vec![1, 1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_no_zeroes() {
        let mut nums = vec![1, 2, 3];
        let expected = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_all_zeroes() {
        let mut nums = vec![0, 0, 0];
        let expected = vec![0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_empty_array() {
        let mut nums: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}
