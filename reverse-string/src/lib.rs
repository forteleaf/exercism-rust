pub fn reverse(input: &str) -> String {
    let input_input = input;
    let mut output = String::new();
    for c in input_input.chars().rev() {
        output.push(c);
    }
    output
}
