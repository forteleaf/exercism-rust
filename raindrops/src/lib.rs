pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
    let mut result = String::new();
    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }
    if result.is_empty() {
        return n.to_string();
    }
    result
}
