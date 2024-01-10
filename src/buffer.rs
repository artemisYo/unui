use std::default::Default;
use crate::pixel::{Pixel, PixelBGRX};

// TODO: Also handle allocating a PixelBuffer in a shm for
//       Wayland rendering; (iffy) required libc functions:
//                          memfd_create, ftruncate, mmap

/// A simple wrapper around a Vec, indexing returns a row as a slice.
#[derive(Default, Debug)]
pub struct PixelBuffer<Format: Pixel> {
    backing: Vec<Format>,
    height: usize,
    width: usize,
}
impl<T: Pixel> PixelBuffer<T> {
    /// Initializes a PixelBuffer with size 0x0
    pub fn new() -> Self { Default::default() }
    /// Initializes a PixelBuffer with the given size, contents are zeroed
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            backing: vec![Default::default(); width * height],
            height,
            width
        }
    }
    /// Resize the PixelBuffer to the given dimensions, invalidating
    /// any content.
    pub fn resize(mut self, new_width: usize, new_height: usize) -> Self {
        self.backing.resize(new_width * new_height, Default::default());
        self.height = new_height;
        self.width = new_width;
        self
    }
    pub fn as_slice(&self) -> &[T] {
        self.backing.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.backing.as_mut_slice()
    }
    pub fn row(&self, index: usize) -> &[T] {
        &self[index]
    }
    pub fn row_mut(&mut self, index: usize) -> &mut [T] {
        &mut self[index]
    }
}
impl PixelBuffer<PixelBGRX> {
    pub fn as_bgrx_slice(&self) -> &[u8] {
        // this is fine as every pixel is already just a [u8;4]
        // if there is unexpected padding, this is bad;
        // however, that should not occur
        unsafe {
            std::slice::from_raw_parts(
                self.as_slice().as_ptr() as *const u8,
                self.backing.len() * 4,
            )
        }
    }
}
impl<T: Pixel> std::ops::Index<usize> for PixelBuffer<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.backing[self.width * index..][..self.width]
    }
}
impl<T: Pixel> std::ops::IndexMut<usize> for PixelBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.backing[self.width * index..][..self.width]
    }
}
