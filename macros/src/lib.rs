#[macro_export]

macro_rules! hashmap {
    ($($key:expr => $value:expr,)*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
    ($($key:expr => $value:expr),*) => {
        crate::hashmap!( $($key => $value,)*)
    };
}
