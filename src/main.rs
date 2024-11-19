mod word_break;
mod word_break_rev;
fn main() {
    let result = word_break::Solution::word_break(
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );
    println!("{:?}", result);
}
