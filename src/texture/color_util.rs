use sdl2::pixels::Color;


/// A struct that for holding RGB values that has these fields contiguously in memory and in the
/// that explicit order. This is so that a pointer to it can be passed to opengl
#[repr(C)]
pub struct SerializedRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl SerializedRGB {
    pub fn new(clr: Color) -> SerializedRGB {
        let red = (clr.r as f32) / (u8::MAX as f32);
        let green = (clr.g as f32) / (u8::MAX as f32);
        let blue = (clr.b as f32) / (u8::MAX as f32);
        SerializedRGB {
            r: red,
            g: green,
            b: blue,
        }
    }
}
