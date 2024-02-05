#[macro_export]

macro_rules! hashmap {
    ($($key:expr => $value:expr,)*) => {
        {
            use ::std::collections::HashMap as hm;
            let mut _map = hm::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
    ($($key:expr => $value:expr),*) => {
        {
            use ::std::collections::HashMap as hm;
            let mut _map = hm::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}
