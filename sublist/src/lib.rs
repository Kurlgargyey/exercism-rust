#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if second_list.len() == 0 || window_checker(first_list, second_list) {
        return Comparison::Superlist;
    } else if first_list.len() == 0 || window_checker(second_list, first_list) {
        return Comparison::Sublist;
    } else {
        return Comparison::Unequal;
    }
}

fn window_checker<T: PartialEq>(candidate_source: &[T], target: &[T]) -> bool {
    candidate_source
        .windows(target.len())
        .any(|window| window == target)
}
