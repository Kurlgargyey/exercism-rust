pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = vec![];
    if n & 0b00001 > 0 {
        result.push("wink")
    }
    if n & 0b00010 > 0 {
        result.push("double blink")
    }
    if n & 0b00100 > 0 {
        result.push("close your eyes")
    }
    if n & 0b01000 > 0 {
        result.push("jump")
    }
    if n & 0b10000 > 0 {
        result.into_iter().rev().collect()
    } else {
        result
    }
}
