use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    let mut shiftgen = key.chars().map(|c| c as i8 - 'a' as i8).cycle();

    shift_string(&mut shiftgen, s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let mut shiftgen = key.chars().map(|c| -(c as i8 - 'a' as i8)).cycle();

    shift_string(&mut shiftgen, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_key();
    let ciphertext = encode(&key, s).unwrap();
    (key, ciphertext)
}

fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new('a', 'z');
    (0..100).map(|_| rng.sample(range)).collect()
}

fn shift_string(shiftgen: &mut impl Iterator<Item = i8>, s: &str) -> Option<String> {
    let mut result = String::new();
    for char in s.chars() {
        let key = shiftgen.next()?;
        if !(key >= -26) || !(key <= 26) {
            return None;
        }
        result.push(cipher_char(char, key))
    }
    Some(result)
}

fn cipher_char(c: char, key: i8) -> char {
    println!("char: {}, key: {}", c, key);
    match c.is_lowercase() {
        true => rotate_char(c, key, 'a'),
        false => rotate_char(c, key, 'A'),
    }
}

fn rotate_char(c: char, key: i8, offset: char) -> char {
    let mut char_cycle = ('a'..='z').cycle();
    let mut new_char_idx = c as i8 - offset as i8 + key;
    if new_char_idx.is_negative() {
        new_char_idx += 26;
    }
    char_cycle.nth(new_char_idx as usize).unwrap()
}
