#[macro_export]

macro_rules! hashmap {
    ($($key:expr => $value:expr),*$(,)?) => {
        //                       ^ this captures a single trailing comma
        {
            use ::std::collections::HashMap as hm;
            let mut temp_map = hm::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
}
