use itertools::*;
#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp_table = Vec::<Vec<Vec<Item>>>::with_capacity();

    let result = (1..=items.len())
        .map(|i| {
            items
                .iter()
                .permutations(i)
                .map(|perm| {
                    perm.into_iter()
                        .map(|item| (item.weight, item.value))
                        .reduce(|acc, item| (acc.0 + item.0, acc.1 + item.1))
                        .unwrap_or((0, 0))
                })
                .filter(|total| total.0 <= max_weight)
                .max_by(|x, y| x.1.cmp(&y.1))
                .unwrap_or((0, 0))
        })
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap_or((0, 0));

    result.1
}
