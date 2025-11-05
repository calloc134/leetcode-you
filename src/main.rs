mod generate_parentheses;
fn main() {
    let n = 2;
    let result = generate_parentheses::Solution::generate_parenthesis(n);
    println!("Generated parentheses for n={}: {:?}", n, result);
}
