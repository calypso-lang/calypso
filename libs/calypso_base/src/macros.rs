#[macro_export]
/// Initialize a `static` list with the values specified.
///
/// # Example
///
///
/// ```rust ignore
/// static_list!(FOO: i32 = [1, 2, 3, 4]);
/// ```
///
/// This expands to:
/// ```rust ignore
/// static FOO: &[i32] = &[1, 2, 3, 4];
/// ```
///
/// This way, you don't have to worry about the number of items
/// in the list or crazy reference things as that's taken care of
/// for you.
macro_rules! static_list {
    ($name:ident: $ty:ty = [$($e:expr),+$(,)?]) => {
        static $name: &[$ty] = &[$($e),+];
    };
}

/// Initialize a [`radix_trie::Trie`](https://docs.rs/radix_trie/0.2.1/radix_trie/struct.Trie.html) with specified values using `lazy_static!`.
/// The trie's value type must be an `enum`.
///
/// # Example
///
/// ```rust ignore
/// enum FooType {
///     Value1, Value2, Bar
/// }
///
/// init_trie!(FOO_TRIE: FooType => {
///     "key1" => Value1,
///     "key2" => Value2,
///     "foo"  => Bar
/// });
/// ```
///
/// This will initialize the trie named `FOO_TRIE` with the key-value pairs `("key1",Value1)`, `("key2",Value2)`, `("foo",Bar)`.
/// This is currently only used in lexing and may be moved to that crate eventually.
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
