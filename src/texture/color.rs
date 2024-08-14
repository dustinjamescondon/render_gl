
/// A struct that for holding RGB values that has these fields contiguously in memory and in the
/// that explicit order. This is so that a pointer to it can be passed to opengl
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SerializedRGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type ColorF32 = SerializedRGB<f32>;
pub type ColorU8 = SerializedRGB<u8>;

impl SerializedRGB<f32> {
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        let red = (r as f32) / (u8::MAX as f32);
        let green = (g as f32) / (u8::MAX as f32);
        let blue = (b as f32) / (u8::MAX as f32);
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }
}

impl<T> SerializedRGB<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
}
