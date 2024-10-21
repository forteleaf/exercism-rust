pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let num_str = num.to_string();
    let num_len = num_str.len();
    let mut sum = 0;
    for c in num_str.chars() {
        sum += c.to_digit(10).unwrap().pow(num_len as u32);
    }
    sum == num
}
