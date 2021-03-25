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
    (pub $name:ident: $ty:ty = [$($e:expr),+$(,)?]) => {
        pub static $name: &[$ty] = &[$($e),+];
    };

    ($name:ident: $ty:ty = [$($e:expr),+$(,)?]) => {
        static $name: &[$ty] = &[$($e),+];
    };
}
