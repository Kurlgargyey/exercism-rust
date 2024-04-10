use gcd;

#[derive(PartialEq, Eq, Debug, Default)]
pub enum Bucket {
    #[default] One,
    Two,
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

    Some(BucketStats::default())
}

fn solvable(capacity_1: u8, capacity_2: u8, goal: u8) -> bool {
    !(goal > capacity_1 && goal > capacity_2) && goal % gcd::binary_u8(capacity_1, capacity_2) == 0
}
