#![feature(bigint_helper_methods)]
pub mod biguint;

/// Define a hash_map! macro to create HashMaps.
#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $value:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
    () => {

    };
}
