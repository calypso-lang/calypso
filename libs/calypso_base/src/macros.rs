#[macro_export]
macro_rules! static_list {
    ($name:ident: $ty:ty = [$($e:expr),+$(,)?]) => {
        static $name: &[$ty] = &[$($e),+];
    };
}

#[macro_export]
macro_rules! init_trie {
    ($name:ident: $triety:ident => { $($key:expr => $value:expr),+ }) => {
        lazy_static! {
            static ref $name: Trie<String, $triety> = {
                use $triety::*;
                let mut t = Trie::new();
                $(t.insert($key.into(), $value));*;
                t
            };
        }
    }
}
