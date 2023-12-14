use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Sendable<T>(pub T);
unsafe impl<T> Send for Sendable<T> {}
impl<T> Deref for Sendable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Sendable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub unsafe fn uncheked_mut<T>(r: &T) -> &mut T {
    (r as *const T as *mut T).as_mut().unwrap()
}
