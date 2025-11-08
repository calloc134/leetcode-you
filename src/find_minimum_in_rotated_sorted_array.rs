pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // 配列の特性上二分探索で探索できる
        // 傾きは右肩上がり
        // 中間要素と右端要素で比較し、中間>右端ならその間に最小値があると判断
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                // 最小値はmidの右側にある
                left = mid + 1;
            } else {
                // 最小値はmidを含む左側にある
                right = mid;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Input: nums = [3,4,5,1,2]
        // Output: 1
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_example_2() {
        // Input: nums = [4,5,6,7,0,1,2]
        // Output: 0
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::find_min(nums), 0);
    }

    #[test]
    fn test_example_3() {
        // Input: nums = [11,13,15,17]
        // Output: 11
        let nums = vec![11, 13, 15, 17];
        assert_eq!(Solution::find_min(nums), 11);
    }
}
