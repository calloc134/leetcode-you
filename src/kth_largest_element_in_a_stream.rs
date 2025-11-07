struct KthLargest {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {}

    fn add(&self, val: i32) -> i32 {}
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
        let kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }

    #[test]
    fn example_2_sequence() {
        let kth = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kth.add(2), 7);
        assert_eq!(kth.add(10), 7);
        assert_eq!(kth.add(9), 7);
        assert_eq!(kth.add(9), 8);
    }
}
