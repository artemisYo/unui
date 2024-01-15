mod xbuffer;
pub use xbuffer::*;
// TODO: Also handle allocating a PixelBuffer in a shm for
//       Wayland rendering; (iffy) required libc functions:
//                          memfd_create, ftruncate, mmap
//       The problem of resizes arises in shm memory:
//         If the Vec needs to be resized, we need to first
//         check if the given shm_pool can accommodate it, otherwise
//         resize the underlying fd to accommodate it as well as the
//         pool itself, then the wl_buffer needs to be destroyed and
//         a new one with the correct size created and only then can
//         the extra memory be used as expected.
//       -> Vec most likely unusable

pub trait Pixel {
	fn set_red(&mut self, _: u8);
	fn set_green(&mut self, _: u8);
	fn set_blue(&mut self, _: u8);
	fn set_alpha(&mut self, _: u8);
}

pub trait PixelBuffer<T: Pixel>
    : std::ops::Index<usize, Output = [T]>
    + std::ops::IndexMut<usize>
{
    /// Initializes a PixelBuffer with the given size, contents are zeroed
	fn with_size(width: usize, height: usize) -> Self;
    /// Resize the PixelBuffer to the given dimensions, invalidating
    /// any content.
	fn resize(self, new_width: usize, new_height: usize) -> Self;
	fn height(&self) -> usize;
	fn width(&self) -> usize;
}
