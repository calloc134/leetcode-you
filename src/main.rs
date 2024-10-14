mod is_subsequence;
fn main() {
    let result = is_subsequence::Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string());
    println!("{:?}", result);
}
