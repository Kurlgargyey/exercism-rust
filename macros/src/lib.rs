#[macro_export]

macro_rules! hashmap {
    ($($key:expr => $value:expr,)*) => {
        {
            use ::std::collections::HashMap as hm;
            let mut temp_map = hm::new();
            $(
                temp_map.insert($key, $value);
            )*
            temp_map
        }
    };
    ($($key:expr => $value:expr),*) => {
        {use macros::hashmap;
        hashmap!( $($key => $value,)*)}
    };
}
