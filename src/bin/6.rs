//The sum of the squares of the first ten natural numbers is,
//
//1^2 + 2^2 + ... + 10^2 = 385
//
//The square of the sum of the first ten natural numbers is,
//
//(1 + 2 + ... + 10)^2 = 55^2 = 3025
//
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.


fn solve(setbound: u64) -> u64 {
    let mut sumofsquares = 0;
    let mut squareofsum = 0;
    if setbound == 0 {
        panic!();
    }
    for i in 1..(setbound + 1) {
        sumofsquares += i * i;
        squareofsum += i;
    }
    (squareofsum * squareofsum) - sumofsquares
}

fn main() {
    println!("{}", solve(100).to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn sum_square_diff_badbound() {
        super::solve(0);
    }
    #[test]
    fn sum_square_diff_10() {
        assert_eq!(2640, super::solve(10));
    }
    #[test]
    fn sum_square_diff_100() {
        assert_eq!(25164150, super::solve(100));
    }
}
