pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    let mut aa: Vec<String> = vec![];
    for i in 0..list.len() {
        if i == list.len() - 1 {
            aa.push(format!("And all for the want of a {}.", list[0]))
        } else {
            aa.push(format!(
                "For want of a {} the {} was lost.",
                list[i],
                list[i + 1]
            ))
        }
    }
    aa.join("\n")
}
