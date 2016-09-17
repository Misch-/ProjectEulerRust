//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

// brute force test numbers until one is found that is divisable by every number up to bound
fn solve(bound: u64) -> u64 {
    let mut i = 0;
    let mut modulus;
    if bound == 0 {
        panic!();
    }
    loop {
        //end result must be divisable by bound, so we can skip a lot of numbers
        i += bound;
        modulus = 1;
        while modulus <= bound {
            if i % modulus == 0 {
                if modulus == bound {
                    return i;
                }    
            }
            else{
                break;
            }
        modulus += 1;
        }
    }
}

fn main() {
    println!("{}", solve(20).to_string());
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn least_common_multiple_0() {
        super::solve(0);
    }
    #[test]
    fn least_common_multiple_1() {
        assert_eq!(1, super::solve(1));
    }
    #[test]
    fn least_common_multiple_10() {
        assert_eq!(2520, super::solve(10));
    }
    #[test]
    fn least_common_multiple_20() {
        assert_eq!(232792560, super::solve(20));
    }
}
