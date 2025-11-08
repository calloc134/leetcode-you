pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        KthLargest { k, nums }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        // まず追加
        self.nums.push(val);
        // 降順ソート
        self.nums.sort_by(|a, b| b.cmp(a));

        // プリントデバッグ
        println!("After adding {}, nums: {:?}", val, self.nums);

        // k番目を返す
        self.nums[(self.k - 1) as usize]
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
