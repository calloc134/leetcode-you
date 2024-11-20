mod valid_parentheses;

fn main() {
    let result = valid_parentheses::Solution::is_valid("()".to_string());
    println!("{}", result);
}
