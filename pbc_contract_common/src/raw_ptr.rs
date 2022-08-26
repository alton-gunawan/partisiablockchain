use std::io::{Read, Write};

/// A wrapper for a raw pointer that allows (unsafe) reading and writing to raw pointers.
pub struct RawPtr {
    offset: usize,
    base: *mut u8,
}

impl RawPtr {
    /// Create a new `RawPtr` instance from the given raw pointer.
    pub fn new(ptr: *mut u8) -> RawPtr {
        RawPtr {
            offset: 0,
            base: ptr,
        }
    }

    /// Return the current offset from the base pointer.
    pub fn get_offset(&self) -> u32 {
        self.offset as u32
    }

    /// Read a single byte from the current pointer position and increment the position.
    pub fn read(&mut self) -> u8 {
        unsafe {
            let val = std::ptr::read(self.base.add(self.offset));
            self.offset += 1;
            val
        }
    }

    /// Write a single byte to the current pointer position and increment the position.
    pub fn write(&mut self, val: u8) {
        unsafe {
            std::ptr::write(self.base.add(self.offset), val);
            self.offset += 1;
        }
    }
}

impl Read for RawPtr {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for buf_item in buf.iter_mut() {
            *buf_item = self.read();
        }

        Ok(buf.len())
    }
}

impl Write for RawPtr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for buf_item in buf.iter() {
            self.write(*buf_item);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
