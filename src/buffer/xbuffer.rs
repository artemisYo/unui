use std::default::Default;

/// A simple wrapper around a Vec, indexing returns a row as a slice.
/// The internal format is BGRX, as that is what xorg uses
#[derive(Default, Debug)]
pub struct XBuffer {
    backing: Vec<BGRX>,
    height: usize,
    width: usize,
}
#[derive(Default, Debug, Clone)]
pub struct BGRX([u8;4]);
impl super::Pixel for BGRX {
    fn set_red(&mut self, r: u8) { self.0[2] = r; }
    fn set_green(&mut self, g: u8) { self.0[1] = g; }
    fn set_blue(&mut self, b: u8) { self.0[0] = b; }
    fn set_alpha(&mut self, a: u8) { self.0[3] = a; }
}
impl super::PixelBuffer<BGRX> for XBuffer {
    fn with_size(width: usize, height: usize) -> Self {
        Self {
            backing: vec![Default::default(); height * width],
            width, height
        }
    }
    fn resize(mut self, new_width: usize, new_height: usize) -> Self {
        self.backing.resize(new_width * new_height, Default::default());
        self.height = new_height;
        self.width = new_width;
        self
    }
    fn height(&self) -> usize {
        self.height
    }
    fn width(&self) -> usize {
        self.width
    }
}
impl XBuffer {
    pub fn as_slice(&self) -> &[u8] {
        // this is fine as every pixel is already just a [u8;4]
        // if there is unexpected padding, this is bad;
        // however, that should not occur
        unsafe {
            std::slice::from_raw_parts(
                self.backing.as_slice().as_ptr() as *const u8,
                self.backing.len() * 4,
            )
        }
    }
}
impl std::ops::Index<usize> for XBuffer {
    type Output = [BGRX];
    fn index(&self, index: usize) -> &Self::Output {
        &self.backing[self.width * index..][..self.width]
    }
}
impl std::ops::IndexMut<usize> for XBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.backing[self.width * index..][..self.width]
    }
}
