pub trait Pixel:
    std::fmt::Debug +
    std::default::Default +
    Clone +
    Copy
{
    type Data;
    fn data(&self) -> &Self::Data;
    fn data_mut(&mut self) -> &mut Self::Data;
}
// channel accessors
pub struct Red;
pub struct Green;
pub struct Blue;
pub struct Alpha;

#[derive(Default, Debug, Clone, Copy)]
// this is probably unecessary but eh
#[repr(align(4))]
pub struct PixelBGRX {
    colors: [u8;4]
}
impl Pixel for PixelBGRX {
    type Data = [u8;4];
    fn data(&self) -> &Self::Data { &self.colors }
    fn data_mut(&mut self) -> &mut Self::Data { &mut self.colors }
}
generateAccessor!(Blue, 0 for PixelBGRX);
generateAccessor!(Green, 1 for PixelBGRX);
generateAccessor!(Red, 2 for PixelBGRX);
generateAccessor!(Alpha, 3 for PixelBGRX);

#[derive(Default, Debug, Clone, Copy)]
#[repr(align(4))]
pub struct PixelXRGB {
    colors: [u8;4]
}
impl Pixel for PixelXRGB {
    type Data = [u8;4];
    fn data(&self) -> &Self::Data { &self.colors }
    fn data_mut(&mut self) -> &mut Self::Data { &mut self.colors }
}
generateAccessor!(Alpha, 0 for PixelXRGB);
generateAccessor!(Red, 1 for PixelXRGB);
generateAccessor!(Green, 2 for PixelXRGB);
generateAccessor!(Blue, 3 for PixelXRGB);

macro_rules! generateAccessor {
    ($name:path, $index:literal for $pixel:path) => {
        impl std::ops::Index<$name> for $pixel {
            type Output = u8;
            fn index(&self, _: $name) -> &Self::Output {
                &self.data()[$index]
            }
        }
        impl std::ops::IndexMut<$name> for $pixel {
            fn index_mut(&mut self, _: $name) -> &mut Self::Output {
                &mut self.data_mut()[$index]
            }
        }
    }
}
