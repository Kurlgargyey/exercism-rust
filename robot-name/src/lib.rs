use std::sync::Mutex;

use rand::Rng;

pub struct Robot(String);

static TAKEN_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());

impl Robot {
    pub fn new() -> Self {
        let name = generate_name();
        Robot(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        let new_name = generate_name();
        free_name(&self.0);
        self.0 = new_name;
    }
}

fn generate_name() -> String {
    let mut name = random_name();
    while TAKEN_NAMES.lock().unwrap().contains(&name) {
        name = random_name();
    }
    block_name(&name);
    println!("Taken names: {:?}", TAKEN_NAMES.lock().unwrap());
    name
}

fn random_name() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}{}{:03}",
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(0..=999)
    )
}

fn free_name(free_name: &String) {
    TAKEN_NAMES.lock()
        .unwrap()
        .retain(|name| name != free_name);
}

fn block_name(blocked_name: &String) {
    TAKEN_NAMES.lock().unwrap().push(blocked_name.clone());
}
