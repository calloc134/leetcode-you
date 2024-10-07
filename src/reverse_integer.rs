pub struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        if (x == -2147483648) {
            return 0;
        }

        let is_negative = x < 0;

        let mut tmp = x.abs();

        let mut num_stack = Vec::new();

        while tmp > 0 {
            num_stack.push(tmp % 10);
            tmp /= 10;
        }

        // i32からオーバーフローしていたら0を返す
        if num_stack.len() > 10 {
            return 0;
        }

        if num_stack.len() == 10 {
            if num_stack[0] > 2 {
                return 0;
            }
            if num_stack[0] == 2 && num_stack[1] > 1 {
                return 0;
            }
            if num_stack[0] == 2 && num_stack[1] == 1 && num_stack[2] > 4 {
                return 0;
            }
            if num_stack[0] == 2 && num_stack[1] == 1 && num_stack[2] == 4 && num_stack[3] > 7 {
                return 0;
            }
            if num_stack[0] == 2
                && num_stack[1] == 1
                && num_stack[2] == 4
                && num_stack[3] == 7
                && num_stack[4] > 4
            {
                return 0;
            }
            if num_stack[0] == 2
                && num_stack[1] == 1
                && num_stack[2] == 4
                && num_stack[3] == 7
                && num_stack[4] == 4
                && num_stack[5] > 8
            {
                return 0;
            }
            if num_stack[0] == 2
                && num_stack[1] == 1
                && num_stack[2] == 4
                && num_stack[3] == 7
                && num_stack[4] == 4
                && num_stack[5] == 8
                && num_stack[6] > 3
            {
                return 0;
            }
            if num_stack[0] == 2
                && num_stack[1] == 1
                && num_stack[2] == 4
                && num_stack[3] == 7
                && num_stack[4] == 4
                && num_stack[5] == 8
                && num_stack[6] == 3
                && num_stack[7] > 6
            {
                return 0;
            }
            if num_stack[0] == 2
                && num_stack[1] == 1
                && num_stack[2] == 4
                && num_stack[3] == 7
                && num_stack[4] == 4
                && num_stack[5] == 8
                && num_stack[6] == 3
                && num_stack[7] == 6
                && num_stack[8] > 4
            {
                return 0;
            }

            if is_negative {
                if num_stack[0] == 2
                    && num_stack[1] == 1
                    && num_stack[2] == 4
                    && num_stack[3] == 7
                    && num_stack[4] == 4
                    && num_stack[5] == 8
                    && num_stack[6] == 3
                    && num_stack[7] == 6
                    && num_stack[8] == 4
                    && num_stack[9] > 8
                {
                    return 0;
                }
            } else {
                if num_stack[0] == 2
                    && num_stack[1] == 1
                    && num_stack[2] == 4
                    && num_stack[3] == 7
                    && num_stack[4] == 4
                    && num_stack[5] == 8
                    && num_stack[6] == 3
                    && num_stack[7] == 6
                    && num_stack[8] == 4
                    && num_stack[9] > 7
                {
                    return 0;
                }
            }
        }

        for i in 0..num_stack.len() {
            let add_num = num_stack[i] * 10_i32.pow((num_stack.len() - i - 1) as u32);
            tmp += add_num;
        }

        if is_negative {
            tmp *= -1;
        }

        if tmp > i32::MAX || tmp < i32::MIN {
            return 0;
        }

        return tmp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = 123;
        let result = 321;
        assert_eq!(Solution::reverse(x), result);
    }

    #[test]
    fn test_2() {
        let x = -123;
        let result = -321;
        assert_eq!(Solution::reverse(x), result);
    }

    #[test]
    fn test_3() {
        let x = 120;
        let result = 21;
        assert_eq!(Solution::reverse(x), result);
    }

    #[test]
    fn test_4() {
        let x = 0;
        let result = 0;
        assert_eq!(Solution::reverse(x), result);
    }

    #[test]
    fn test_5() {
        let x = 1534236469;
        let result = 0;
        assert_eq!(Solution::reverse(x), result);
    }

    #[test]
    fn test_6() {
        let x = -2147483648;
        let result = 0;
        assert_eq!(Solution::reverse(x), result);
    }
}
