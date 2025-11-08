use std::collections::BinaryHeap;

pub struct KthLargest {
    k: i32,
    nums: BinaryHeap<i32>,
    // 要素数をk個で制限するという運用
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        // 要素数がk未満の場合、MINで埋める
        let mut nums = nums;
        while nums.len() < k as usize {
            nums.push(i32::MIN);
        }

        KthLargest {
            k,
            nums: BinaryHeap::from(nums),
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        println!("Before adding {}, nums: {:?}", val, self.nums);
        // 二分ヒープ(降順)の一番小さい要素より大きい場合には追加
        if 
        
            println!("Adding value: {}", val);
            self.nums.push(val);
        }
        // k番目を返す
        let mut result = self.nums.clone().into_sorted_vec();
        // 逆順にする
        println!("Current nums (sorted before reverse): {:?}", result);
        result.reverse();
        println!("Current nums (sorted): {:?}", result);
        result[self.k as usize - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_sequence() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }

    #[test]
    fn example_2_sequence() {
        let mut kth = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kth.add(2), 7);
        assert_eq!(kth.add(10), 7);
        assert_eq!(kth.add(9), 7);
        assert_eq!(kth.add(9), 8);
    }
}
