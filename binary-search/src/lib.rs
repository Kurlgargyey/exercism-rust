pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut lower_bound = 0;
    let mut upper_bound = array.len();
    let get_idx = || (upper_bound - lower_bound) / 2 - 1;
    let mut idx = get_idx();
    loop {
        let middle_value = array[idx];
        match middle_value {
            x if x > key => {
                upper_bound = idx + 1;
                idx = get_idx();
            }
            x if x < key => {
                lower_bound = idx + 1;
                idx = get_idx();
            }
            _ => {
                return Some(idx);
            }
        }
        if lower_bound > upper_bound {
            return None;
        }
    }
}
