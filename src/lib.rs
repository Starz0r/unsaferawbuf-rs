use std::mem::size_of;

use sealed::sealed;

pub struct UnsafeRawBuf {
    pub cursor: *mut isize,
    start: *mut isize,
}

#[sealed]
pub trait UnsafeRawBufReader {
	unsafe fn read<T>(&mut self) -> T
	where
		T: Copy; // TODO: eliminate this if possible
}

#[sealed]
pub trait UnsafeRawBufWriter {
	unsafe fn write<T>(&mut self, val: T);
}

#[sealed]
impl UnsafeRawBufReader for UnsafeRawBuf {
    unsafe fn read<T>(&mut self) -> T
    where
        T: Copy,
    {
        let data: *mut T = self.cursor.cast();

        // increment cursor
        let mut next_pos = self.cursor.clone() as i64;
        next_pos += size_of::<T>() as i64;
        self.cursor = next_pos as usize as *mut isize;
        *data // DANGER: unsafe!!!!
    }
}

#[sealed]
impl UnsafeRawBufWriter for UnsafeRawBuf {
    unsafe fn write<T>(&mut self, val: T) {
        // generically reinterpret_cast and write to that pointer
        let ptr: *mut T = self.cursor.cast();
        ptr.write(val);

        // increment cursor
        let mut next_pos = self.cursor.clone() as i64;
        next_pos += size_of::<T>() as i64;
        self.cursor = next_pos as usize as *mut isize;
    }
}

impl UnsafeRawBuf {
    pub const unsafe fn new_uninit() -> Self {
        Self {
            cursor: 0 as _,
            start: 0 as _,
        }
    }

    pub const unsafe fn from_address(addr: *mut isize) -> Self {
        Self {
            cursor: addr,
            start: addr,
        }
    }

    pub unsafe fn reset(&mut self) {
        self.cursor = self.start;
    }

    pub unsafe fn set_location(&mut self, addr: *mut isize) {
        self.cursor = addr;
        self.start = self.cursor;
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use super::*;
    use libc::{malloc, memset};

    #[test]
    fn read_and_write() {
        let rawbuf = unsafe { malloc(32) };
        assert_ne!(rawbuf, null_mut());

        unsafe {
            memset(rawbuf, 0, 32);
        }

        let mut buf = unsafe { UnsafeRawBuf::from_address(rawbuf as _) };
        unsafe {
            buf.write(42);
            buf.reset();
        };

        assert_eq!(42, unsafe { buf.read::<i32>() });
    }
}
