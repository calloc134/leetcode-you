mod romen_to_integer;
fn main() {
    let romen = "MCMXCIV".to_string();
    let result = romen_to_integer::Solution::roman_to_int(romen);
    println!("{:?}", result);
}
