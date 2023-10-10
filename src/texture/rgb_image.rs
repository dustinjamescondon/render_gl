use image::{RgbImage, ImageBuffer};
use sdl2::pixels::Color;

/// Similiar to RGBTexture, but it's independent of opengl, so it can easily 
/// be used on a seperate thread
pub struct RGBImage
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>
}

impl RGBImage
{
    pub fn new(data: Vec<Color>, width: usize, height: usize) -> RGBImage {
        RGBImage {
            width,
            height,
            data,
        }
    }
}

impl RGBImage {
    /// TODO Need to get rid of arbitrary methods like this and 
    /// do some explicit transformation steps
    pub fn to_rgb_image_flipped_y(&self) -> RgbImage {
        let width_u32 = self.width as u32;
        let height_u32 = self.height as u32;

        let img: RgbImage = ImageBuffer::from_fn(
            width_u32,
            height_u32,
            |x, y| {
                let xi = x as usize;
                let yi = (height_u32 - y - 1) as usize;
                let tex_width = self.width;
                let rgb = self.data[yi * tex_width + xi];
                image::Rgb([rgb.r, rgb.g, rgb.b])
            },
        );

        img
    }


    #[allow(dead_code)]
    pub fn to_rgb_image(&self) -> RgbImage {
        let width_u32 = self.width as u32;
        let height_u32 = self.height as u32;

        let img: RgbImage = ImageBuffer::from_fn(
            width_u32,
            height_u32,
            |x, y| {
                let xi = x as usize;
                let yi = y as usize;
                let tex_width = self.width;
                let rgb = self.data[yi * tex_width + xi];
                image::Rgb([rgb.r, rgb.g, rgb.b])
            },
        );

        img
    }
}