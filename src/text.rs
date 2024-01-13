extern crate rusttype;
use nalgebra::Vector2;
use std::{
    collections::HashMap,
    ffi::{c_void, CString},
};
type Vector2i = Vector2<i32>;
use crate::{gl_panic, glchk, REDTexture, Texture};
use freetype as ft;

pub struct Character {
    pub texture: REDTexture,
    pub size: Vector2i,
    pub bearing: Vector2i,
    pub advance: u32,
}

pub struct FontContext {
    pub map: HashMap<char, Character>,
}

pub fn init_font(font: &str, char_width: isize) -> Result<FontContext, ()> {
    let font_path = std::env::current_dir().unwrap().join(font);
    let data = std::fs::read(&font_path).unwrap();
    let lib = ft::Library::init().unwrap();
    let face = lib.new_face(font, 0).unwrap();
    face.set_char_size(char_width, 0, 50, 0).unwrap();

    let mut map = HashMap::<char, Character>::new();
    for i in 0_u8..128_u8 {

        face.load_char(i as usize, ft::face::LoadFlag::RENDER).unwrap();
        let glyph = face.glyph();

        let texture = REDTexture::new_from_data(
            glyph.bitmap().raw().buffer as *mut gl::types::GLvoid,
            glyph.bitmap().width() as usize,
            glyph.bitmap().rows() as usize,
        );

        texture.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        gl_panic!();

        texture.unbind();
        let character = Character {
            texture,
            size: Vector2i::new(glyph.bitmap().width(), glyph.bitmap().rows()),
            bearing: Vector2i::new(glyph.bitmap_left(), glyph.bitmap_top()),
            advance: glyph.advance().x as u32,
        };
        map.insert(i as char, character);
    }

    Ok(FontContext { map })
}

#[cfg(test)]
mod test {
    use super::*;
    use freetype as ft;

    /// Just makes sure I'm getting a bitmap out of it
    #[test]
    fn freetype_rs() {
        let lib = ft::Library::init().unwrap();
        let face = lib.new_face("res/FreeSansBold.ttf", 0).unwrap();
        face.set_char_size(40 * 64, 0, 50, 0).unwrap();
        face.load_char('f' as usize, ft::face::LoadFlag::RENDER)
            .unwrap();
        let glyph = face.glyph();
        assert!(glyph.bitmap().width() > 0);
        assert!(glyph.bitmap().rows() > 0);
    }
}
