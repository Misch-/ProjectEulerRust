//A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
//Find the largest palindrome made from the product of two 3-digit numbers.

// Loop through from 999 to 100 for both multipliers, testing if result is palindrome, return the maximum result
fn solve() -> u64 {
    let mut num1 = 999;
    let mut num2 = 999;
    let mut result;
    let mut result_str: String;
    let mut result_str_rev: String;
    let mut result_max = 0;
    while num1 > 100 {
        while num2 > 100 {
            //Is num2 * num2 result a palindrome?
            result_str = (num1 * num2).to_string();
            result_str_rev = result_str.chars().rev().collect();
            if result_str == result_str_rev {
                //Is it larger than any we've found previously?
                result = result_str.parse::<u64>().unwrap();
                if result > result_max {
                    result_max = result;
                }
            }
            num2 -= 1;
        }
    num2 = 999;
    num1 -= 1;
    }
    //Return largest found palindrome
    result_max
}

fn main() {
    println!("{}", solve().to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    fn max_palindrome_3_digits() {
        assert_eq!(906609, super::solve());
    }
}
