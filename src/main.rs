mod integer_to_romen;

fn main() {
    let num = 3749;
    let result = integer_to_romen::Solution::int_to_roman(num);
    println!("{:?}", result);
}
