mod container_with_most_water;
fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = container_with_most_water::Solution::max_area(height);
    println!("{:?}", result);
}
