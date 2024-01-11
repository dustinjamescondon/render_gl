extern crate freetype;
use std::{collections::HashMap, ffi::c_void};
use nalgebra::Vector2;
type Vector2i = Vector2<i32>;

use freetype::{face::LoadFlag, Face};

use crate::{REDTexture, Texture};

pub struct Character {
    pub texture: REDTexture,
    pub size: Vector2i,
    pub bearing: Vector2i,
    pub advance: u32,
}

pub struct FontContext {
    pub map: HashMap<char, Character>,
}

pub fn init_font(font: &str) -> Result<FontContext, freetype::Error> {
    let ft = freetype::Library::init().expect("Couldn't init freetype");

    unsafe {
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    }

    let face = ft.new_face(font, 0)?;

    let mut map = HashMap::<char, Character>::new();
    for i in 0_u8..128_u8 {
        face.load_char(i as usize, LoadFlag::RENDER).unwrap();
        let glyph = face.glyph();
        let texture = REDTexture::new_from_data(
            glyph.bitmap().buffer().as_ptr() as *mut gl::types::GLvoid,
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
        texture.unbind();

        let character = Character {
            texture,
            size: Vector2i::new(glyph.bitmap().width(), glyph.bitmap().rows()),
            bearing: Vector2i::new(glyph.bitmap_left(), glyph.bitmap_top()),
            advance: glyph.advance().x as u32,
        };
        map.insert(i as char, character);
    }

    Ok(FontContext {
        map,
    })
}
