pub fn factorial(n: u64) -> u64 {
    if n < 2 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
