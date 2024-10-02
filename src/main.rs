mod add_two_numbers;
mod longuest_palindromic;
mod longuest_sub;
mod palindrome;
mod two_sum;

fn main() {
    // let result = two_sum::Solution::two_sum(nums, target);
    // let result = palindrome::Solution::is_palindrome(121);
    // println!("{:?}", result);

    // let target = "pwwkew".to_string();
    // let result = longuest_sub::Solution::length_of_longest_substring(target);

    let target = "babad".to_string();
    let result = longuest_palindromic::Solution::longest_palindrome(target);
    println!("{:?}", result);
}
