pub struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left_pointer = 0;
        let mut right_pointer = height.len() - 1;

        while left_pointer < right_pointer {
            let left_bar = height[left_pointer];
            let right_bar = height[right_pointer];
            let hight = if left_bar < right_bar {
                left_bar
            } else {
                right_bar
            };

            let width = right_pointer - left_pointer;
            let square = hight * width as i32;

            if square > result {
                result = square;
            }

            if left_bar < right_bar {
                left_pointer += 1;
            } else {
                right_pointer -= 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test_2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
