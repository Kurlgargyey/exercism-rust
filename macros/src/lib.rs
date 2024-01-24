#[macro_export]
use std::collections::HashMap;
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}
