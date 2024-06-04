use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut shiftgen = key.chars().map(|c| (c as i64 - 'a' as i64) as i8).cycle();

    shift_string(&mut shiftgen, s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    let mut shiftgen = key
        .chars()
        .map(|c| -((c as i64 - 'a' as i64) as i8))
        .cycle();

    shift_string(&mut shiftgen, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_key();
    let cipher = encode(&key, s).unwrap();
    (key, cipher)
}

fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let range = rand::distributions::Uniform::new('a', 'z');
    (0..100).map(|_| rng.sample(range)).collect()
}

fn shift_string(shiftgen: &mut impl Iterator<Item = i8>, s: &str) -> Option<String> {
    let mut result = String::new();
    for char in s.chars() {
        let key = shiftgen.next().expect("key ended for some reason");
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
        new_char_idx = 26 + new_char_idx
    }
    char_cycle
        .nth(new_char_idx as usize)
        .expect("something went wrong")
}
