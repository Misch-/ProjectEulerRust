//The prime factors of 13195 are 5, 7, 13 and 29.
//
//What is the largest prime factor of the number 600851475143 ?

// Loop through 0 to sqrt of composite number looking for factors, if found divide composite by factor and recurse until none are left.
fn solve(mut num: u64) -> u64 {
    let mut i: u64 = 2;
    let sqrt: u64 = (num as f64).sqrt() as u64 + 1;
    while i < (sqrt) {
        if num % i == 0 {
            //if result is one, num is already prime
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

#[cfg(test)]
mod tests {
    #[test]
    fn factor_13195() {
        assert_eq!(29, super::solve(13195));
    }
    #[test]
    fn factor_600851475143() {
        assert_eq!(6857, super::solve(600851475143));
    }
}
