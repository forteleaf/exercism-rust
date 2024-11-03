pub fn square_of_sum(n: u32) -> u32 {
    // (n * (n + 1) / 2).pow(2)

    let sum: u32 = (1..=n).sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // n * (n + 1) * (2 * n + 1) / 6
    (1..=n).map(|x| x * x).sum()
}

pub fn difference(n: u32) -> u32 {
    let a: u32 = square_of_sum(n);
    let b: u32 = sum_of_squares(n);

    a.abs_diff(b)
}
