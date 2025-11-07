use crate::kth_largest_element_in_a_stream::KthLargest;

mod kth_largest_element_in_a_stream;
fn main() {
    let kth = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", kth.add(3));
}
