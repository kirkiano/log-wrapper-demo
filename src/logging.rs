
/// Our trusty wrapper
#[derive(Debug, Copy, Clone)]
pub struct Logging<T>(T);


impl<T> Logging<T> {

    /// constructor
    pub fn new(t: T) -> Self {
        Self(t)
    }

    /// immutable access to the wrapped thing
    pub fn inner(&self) -> &T {
        &self.0
    }
}
