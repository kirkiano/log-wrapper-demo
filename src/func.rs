
use crate::traits::F;


/// Thanks to trait [`F`], this function can be polymorphic.
pub fn some_function<T: F>(t: &T) {
    /* ... */
    let _x = t.f();
    /* ... */
}
