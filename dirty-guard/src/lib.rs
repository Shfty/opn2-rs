use std::ops::{Deref, DerefMut};

/// Wrapper type for tracking changes to a contained value
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DirtyGuard<T>
where
    T: Clone + Eq,
{
    dirty: bool,
    data: T,
}

#[allow(dead_code)]
impl<T> DirtyGuard<T>
where
    T: Clone + Eq,
{
    pub fn new(data: T) -> Self {
        DirtyGuard { dirty: false, data }
    }

    /// Reads the contained data, ignoring the dirty flag
    pub fn read(&self) -> &T {
        &self.data
    }

    /// If the dirty flag is set, resets it and returns Some(T). Else, returns None.
    pub fn try_read(&mut self) -> Option<&T> {
        match self.dirty {
            true => {
                self.dirty = false;
                Some(&self.data)
            }
            false => None,
        }
    }

    /// Returns a DirtyGuardRef to the contained data
    pub fn write(&mut self) -> DirtyGuardRef<T> {
        let data = self.data.clone();
        DirtyGuardRef { guard: self, data }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

/// Mutable reference to data inside a DirtyGuard, will update the contained value and dirty flag on drop
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DirtyGuardRef<'a, T>
where
    T: Clone + Eq,
{
    guard: &'a mut DirtyGuard<T>,
    data: T,
}

#[allow(dead_code)]
impl<'a, T> DirtyGuardRef<'a, T>
where
    T: Clone + Eq,
{
    pub fn get(&self) -> &T {
        Deref::deref(self)
    }

    pub fn get_mut(&mut self) -> &mut T {
        DerefMut::deref_mut(self)
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }
}

impl<'a, T> Deref for DirtyGuardRef<'a, T>
where
    T: Clone + Eq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> DerefMut for DirtyGuardRef<'a, T>
where
    T: Clone + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Drop for DirtyGuardRef<'a, T>
where
    T: Clone + Eq,
{
    fn drop(&mut self) {
        if self.data != self.guard.data {
            self.guard.data = self.data.clone();
            self.guard.dirty = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_guard() {
        let mut guard = DirtyGuard::new("Hello");
        let mut data = guard.write();
        *data = "Goodbye";
        drop(data);
        assert!(guard.is_dirty());
        println!("Guard: {:?}", guard);
        assert!(guard.try_read() == Some(&"Goodbye"))
    }
}
