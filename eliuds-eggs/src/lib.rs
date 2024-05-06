pub fn egg_count(display_value: u32) -> usize {
    let mut result: usize = 0;
    let mut display_value = display_value;
    while display_value > 0 {
        match display_value % 2 {
            0 => (),
            _ => result += 1,
        }
        display_value /= 2;
    }
    result
}
