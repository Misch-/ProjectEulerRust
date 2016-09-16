fn fibonacci(nthterm: u32) -> u32 {
    if nthterm <= 2 {
        return nthterm;
    }
    fibonacci(nthterm - 1) + fibonacci(nthterm - 2)
}

fn solve(bound: u32) -> u32 {
    let mut i = 0;
    let mut sum = 0;
    while fibonacci(i) < bound {
        i += 1;
        if fibonacci(i) %2 == 0 {
            sum += fibonacci(i);
        }
    }
    sum
}

fn main() {
    println!("{}", solve(4000000).to_string());
}
