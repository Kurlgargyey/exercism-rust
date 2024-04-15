use std::sync::Mutex;

use rand::Rng;

pub struct Robot(String);

const TAKEN_NAMES: Mutex<Vec<String>> = Mutex::new(Vec::<String>::new());

impl Robot {
    pub fn new() -> Self {
        let name = generate_name();
        println!("{}", name);
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
    let mut rng = rand::thread_rng();
    let mut name = format!(
        "{}{}{:03}",
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(0..=999)
    );
    while TAKEN_NAMES.lock().unwrap().contains(&name) {
        name = generate_name();
    }
    block_name(&name);
    name
}

fn free_name(free_name: &String) {
    TAKEN_NAMES.lock()
        .unwrap()
        .retain(|name| name != free_name);
}

fn block_name(blocked_name: &String) {
    TAKEN_NAMES.lock().unwrap().push(blocked_name.clone());
}
