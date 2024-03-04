pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut lower_bound: isize = 0;
    let mut upper_bound: isize = (array.len() - 1) as isize;
    while lower_bound <= upper_bound {
        let idx = lower_bound + (upper_bound - lower_bound) / 2;
        let middle_value = array[idx as usize];
        match middle_value {
            x if x > key => {
                upper_bound = idx - 1;
            }
            x if x < key => {
                lower_bound = idx + 1;
            }
            _ => {
                return Some(idx as usize);
            }
        }
    }
    None
}
