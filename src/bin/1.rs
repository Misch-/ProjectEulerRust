//If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//
//Find the sum of all the multiples of 3 or 5 below 1000.

//Returns sum of all multiples of 3 or 5 below given bound
fn solve(bound: u32) -> u32 {
    (1..bound)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn main() {
    println!("{}", solve(1000).to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_10() {
        assert_eq!(23, super::solve(10));
    }
    #[test]
    fn sum_below_1000() {
        assert_eq!(233168, super::solve(1000));
    }
}
