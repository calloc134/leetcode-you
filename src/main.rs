mod add_two_numbers;
mod longuest_palindromic;
mod longuest_sub;
mod palindrome;
mod reverse_integer;
mod two_sum;
mod zigzag;
fn main() {
    // let result = two_sum::Solution::two_sum(nums, target);
    // let result = palindrome::Solution::is_palindrome(121);
    // println!("{:?}", result);

    // let target = "pwwkew".to_string();
    // let result = longuest_sub::Solution::length_of_longest_substring(target);

    // let target = "babad".to_string();
    // let result = longuest_palindromic::Solution::longest_palindrome(target);

    // let s = "PAYPALISHIRING".to_string();
    // let num_rows = 3;
    // let result = zigzag::Solution::convert(s, num_rows);

    let target = -2147483648;
    let result = reverse_integer::Solution::reverse(target);
    println!("{:?}", result);
}
