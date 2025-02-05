use image::{RgbImage, ImageBuffer};

use super::{ColorF32, ColorU8, RGBTexture};

/// Similiar to RGBTexture, but it's independent of opengl, so it can easily 
/// be used on a seperate thread
pub struct RGBImage
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<ColorU8>
}

impl RGBImage
{
    pub fn new_u8(data: Vec<ColorU8>, width: usize, height: usize) -> RGBImage {
        RGBImage {
            width,
            height,
            data,
        }
    }

    pub fn new_f32(data_f32: Vec<ColorF32>, width: usize, height: usize) -> RGBImage {
        let data = data_f32.iter().map(|x| x.into_u8()).collect();

        RGBImage {
            width,
            height,
            data,
        }
    }

    pub fn to_rgb_texture(&self) -> RGBTexture {
        let data_f32: Vec<[f32;3]> = self.data.iter().map(|clr| clr.into_f32().into()).collect();
        RGBTexture::new_from_data(data_f32.as_slice(), self.width, self.height)
    }
}

impl RGBImage {

    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[ColorU8]> {
        self.data.chunks(self.width)
    }
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