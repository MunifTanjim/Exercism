#[macro_export]
macro_rules! hashmap {
    () => {
        crate::HashMap::new()
    };
    ( $( $k:expr => $v:expr ),+ $(,)? ) => {
        {
            let mut hm = crate::HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}
