
use crate::traits::F;


/// Some struct that implements [`F`] below.
#[derive(Debug, Clone)]
pub struct C;


impl C {
    pub fn foo(&self) -> i32 {
        42
    }
}


impl F for C {
    fn f(&self) -> i32 {
        self.foo()
    }
}
