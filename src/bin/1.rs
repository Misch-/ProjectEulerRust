fn solve(bound: u32) -> u32 {
    (1..bound)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

fn main() {
    println!("{}", solve(1000).to_string());
}
