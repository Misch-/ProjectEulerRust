//The prime factors of 13195 are 5, 7, 13 and 29.
//
//What is the largest prime factor of the number 600851475143 ?

fn solve(mut num: u64) -> u64 {
    let mut i: u64 = 2;
    let sqrt: u64 = (num as f64).sqrt() as u64 + 1;
    while i < (sqrt) {
        if num % i == 0 {
            if num / i != 1 {
                num = num / i;
                i = 2;
            }
        }
        i += 1;
    }
    num
}

fn main() {
    println!("{}", solve(600851475143).to_string());
}
