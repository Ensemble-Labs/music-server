/// A macro to generate getters for every field of a struct.
///
/// Example:
/// ```rs
/// pub struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
/// make_getters!(MyStruct, field1: String, field2: i32);
/// ```
#[macro_export]
macro_rules! make_getters {
    ($s:ident,$($x:ident:$t:ty),+) => { // dispatcher using recursion to emulate switch
        impl $s {
            $($crate::make_getters!{ $x:$t })*
        }
    };
    ($x:ident:String) => { // special case 1: if referencing a `String`, return `&str` not `&String`
        pub fn $x(&self) -> &str {
            &self.$x
        }
    };
    ($x:ident:Vec<$t:ty>) => { // special case 2: instead of `&Vec<T>`, return `&[T]`
        pub fn $x(&self) -> &[$t] {
            &self.$x
        }
    };
    ($x:ident:$t:ty) => { // default case: just return `&T`
        pub fn $x(&self) -> &$t {
            &self.$x
        }
    };
}
