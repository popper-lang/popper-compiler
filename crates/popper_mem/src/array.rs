use std::ffi::{c_void, CString};
use std::ptr::NonNull;
use libc::malloc;
use libc::free;

pub struct RawArray<T> {
    data: NonNull<T>,
    len: usize,
}

impl<T> RawArray<T> {

    ///
    /// # Safety
    ///
    /// This function is unsafe because the pointer can be null
    pub unsafe fn new_raw_unchecked(data: *mut T, len: usize) -> Self {
        RawArray { data: NonNull::new_unchecked(data), len }
    }

    pub fn alloc(length: usize) -> Option<Self> {
        let data = unsafe {
            malloc(length * std::mem::size_of::<T>()) as *mut T
        };
        Some(RawArray { data: NonNull::new(data)?, len: length })
    }

    pub fn from_slice(slice: &[T]) -> Option<Self> {
        let data = unsafe { malloc(std::mem::size_of_val(slice)) as *mut T };
        unsafe { std::ptr::copy_nonoverlapping(slice.as_ptr(), data, slice.len()) };
        Some(RawArray { data: NonNull::new(data)?, len: slice.len() })
    }

    pub fn from_vec(vec: Vec<T>) -> Option<Self> {
        let data = unsafe { malloc(std::mem::size_of_val(&vec)) as *mut T};
        let length = vec.len();
        unsafe { std::ptr::copy(vec.as_ptr(), data, length) };
        
        Some(RawArray { data: NonNull::new(data)?, len: length })
    }

    pub fn as_raw_ptr(&self) -> *mut T {
        self.data.as_ptr()
    }

    ///
    /// # Safety
    ///
    /// This function is unsafe because it dereferences the pointer
    pub unsafe fn into_slice(self) -> &'static mut  [T] {
        std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl RawArray<i8> {
    pub fn from_str(s: &str) -> Option<Self> {
        let c_str = CString::new(s).unwrap();
        let data = c_str.into_raw();
        Some(RawArray { data: NonNull::new(data)?, len: s.len() })
    }

    pub unsafe fn into_str(self) -> String {
        let c_str = CString::from_raw(self.data.as_ptr());
        c_str.into_string().unwrap()
    }

}


impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.data.as_ptr() as *mut c_void);
        }
    }
}