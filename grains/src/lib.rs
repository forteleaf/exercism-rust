pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");

    if s < 1 || s > 64 {
        panic!("wrong number");
    }

    let mut result: u64 = 1;
    for _ in 1..s {
        result *= 2;
    }
    result
}

pub fn total() -> u64 {
    // todo!();
    (1..=64).map(square).sum()
}
