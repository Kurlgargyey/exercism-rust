use gcd;

#[derive(PartialEq, Eq, Debug, Default)]
pub enum Bucket {
    #[default] One,
    Two,
}

pub struct Jug {
    pub id: Bucket,
    capacity: u8,
    pub content: u8,
}

impl Jug {
    pub fn is_empty(&self) -> bool {
        self.content == 0
    }

    pub fn is_full(&self) -> bool {
        self.content == self.capacity
    }

    pub fn free_capacity(&self) -> u8 {
        self.capacity - self.content
    }

    pub fn fill_up(&mut self) {
        self.content = self.capacity;
    }
    pub fn dump(&mut self) {
        self.content = 0;
    }

    pub fn pour(&mut self, other: &mut Jug) {
        let other_free = other.free_capacity();
        other.content = std::cmp::min(other.capacity, other.content + self.content);
        self.content -= std::cmp::min(other_free, self.content);
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Default)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket
) -> Option<BucketStats> {
    if !solvable(capacity_1, capacity_2, goal) {
        return None;
    }
    use Bucket::*;

    let mut moves = 0;
    let mut jug_1 = Jug { id: One, capacity: capacity_1, content: 0 };
    let mut jug_2 = Jug { id: Two, capacity: capacity_2, content: 0 };

    match start_bucket {
        One => {
            while jug_1.content != goal && jug_2.content != goal {
                moves += 1;
                println!("Move number {}", moves);
                pour_continually(&mut jug_1, &mut jug_2, goal);
            }
        }
        Two => {
            while jug_1.content != goal && jug_2.content != goal {
                moves += 1;
                println!("Move number {}", moves);
                pour_continually(&mut jug_2, &mut jug_1, goal);
            }
        }
    }

    let (goal_bucket, other_bucket) = if jug_1.content == goal {
        (jug_1.id, jug_2.content)
    } else {
        (jug_2.id, jug_1.content)
    };

    Some(BucketStats { moves, goal_bucket, other_bucket })
}

fn solvable(capacity_1: u8, capacity_2: u8, goal: u8) -> bool {
    goal <= std::cmp::max(capacity_1, capacity_2) &&
        goal % gcd::binary_u8(capacity_1, capacity_2) == 0
}

fn pour_continually(service: &mut Jug, target: &mut Jug, goal: u8) {
    if service.is_empty() {
        println!("Service jug ({:?}) is empty!", service.id);
        service.fill_up();
    } else if target.capacity == goal {
        println!("Target jug capacity ({:?}) fits the goal!", service.id);
        target.fill_up()
    } else if target.is_full() {
        println!("Target jug ({:?}) is full!", target.id);
        target.dump();
    } else {
        println!("Pouring!");
        service.pour(target);
    }
}
