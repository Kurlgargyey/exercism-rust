pub fn build_proverb(list: &[&str]) -> String {
    (0..list.len())
        .step_by(1)
        .map(|i| {
            if !(i == list.len() - 1) {
                format!("For want of a {} the {} was lost.\n", list[i], list[i + 1])
            } else {
                format!("And all for the want of a {}.", list[0])
            }
        })
        .collect::<String>()
}
