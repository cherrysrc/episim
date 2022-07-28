use std::cell::UnsafeCell;

pub struct UnsafeArray<T>(UnsafeCell<Vec<T>>);

unsafe impl<T> Send for UnsafeArray<T> {}
unsafe impl<T> Sync for UnsafeArray<T> {}

impl<T> UnsafeArray<T> {
    pub fn new(vec: Vec<T>) -> UnsafeArray<T> {
        UnsafeArray {
            0: UnsafeCell::new(vec),
        }
    }

    pub fn get(&self) -> &Vec<T> {
        unsafe { &*self.0.get() }
    }

    pub fn get_at_mut(&self, index: usize) -> &mut T {
        unsafe { &mut (*self.0.get())[index] }
    }

    pub fn len(&self) -> usize {
        unsafe { (*self.0.get()).len() }
    }
}
