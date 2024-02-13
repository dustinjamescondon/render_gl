extern crate rusttype;
use nalgebra::Vector2;
use nalgebra_glm::{Vec2, Vec4, Mat4};
use std::{
    collections::HashMap,
    ffi::{c_void, CString},
};
type Vector2i = Vector2<i32>;
use crate::{gl_panic, glchk, ArrayBuffer, Program, REDTexture, Shader, Texture, VertexArray};
use freetype as ft;

pub struct FontContext {
    map: HashMap<char, Character>,
    pub pixel_height: u32,
    text_vao: VertexArray,
    text_vbo: ArrayBuffer,
    text_shader: Program,
}

struct Character {
    pub texture: REDTexture,
    pub size: Vector2i,
    pub bearing: Vector2i,
    pub advance: u32,
}

impl FontContext {
    pub fn new(font: &str, pixel_height: u32) -> Result<FontContext, ()> {
        let lib = ft::Library::init().unwrap();
        let face = lib.new_face(font, 0).unwrap();
        face.set_pixel_sizes(0, pixel_height).unwrap();

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        let mut map = HashMap::<char, Character>::new();
        for i in 0_u8..128_u8 {
            face.load_char(i as usize, ft::face::LoadFlag::RENDER)
                .unwrap();
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

            let character = Character {
                texture,
                size: Vector2i::new(glyph.bitmap().width(), glyph.bitmap().rows()),
                bearing: Vector2i::new(glyph.bitmap_left(), glyph.bitmap_top()),
                advance: glyph.advance().x as u32,
            };
            map.insert(i as char, character);
        }

        let text_vao = VertexArray::new();
        let text_vbo = ArrayBuffer::new();
        text_vao.bind();
        text_vbo.bind();
        text_vbo.dynamic_draw_data(&[0_f32; 6 * 4]);
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4_usize * ::std::mem::size_of::<f32>()) as gl::types::GLsizei,
                0 as *const gl::types::GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            text_vao.unbind();
        }

        gl_panic!();

        Ok(FontContext {
            map,
            pixel_height,
            text_shader: Program::from_src(
                include_str!("res/text_vertex_shader.glsl"),
                include_str!("res/text_frag_shader.glsl")
            ).unwrap(),
            text_vao,
            text_vbo,
        })
    }

    fn text_width(&self, text: &String, scale: f32) -> f32 {
        let mut width = 0_f32;
        for c in text.chars() {
            let ctex = self.map.get(&c).unwrap();
            width += (ctex.advance >> 6) as f32 * scale;
        }

        width
    }

    pub fn render_text_center_justified(&self, text: &String, center_pos: Vec2, scale: f32, projection: &Mat4, clr: &[f32; 3]) {
        let text_width = self.text_width(text, scale);

        let mut corner_pos = center_pos;
        corner_pos.x -= 0.5_f32 * text_width;
        self.render_text(text, corner_pos, scale, projection, clr)
    }

    pub fn render_text(&self, text: &String, pos: Vec2, scale: f32, projection: &Mat4, clr: &[f32; 3]) {
        self.text_shader.set_used();
        self.text_shader
            .set_3float("textColor".to_string(), clr.clone());

        self.text_shader
            .set_uniform_glm_m4("projection".to_string(), &projection);    

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.text_vao.bind();

            let mut _pos = pos.clone();

            // iterate through characters
            for c in text.chars() {
                let ctex = self.map.get(&c).unwrap();

                let xpos = _pos.x + (ctex.bearing.x as f32) * scale;
                let ypos = _pos.y - (ctex.size.y - ctex.bearing.y) as f32 * scale;

                let w = (ctex.size.x as f32) * scale;
                let h = (ctex.size.y as f32) * scale;

                let vertices = [
                    xpos,
                    ypos + h,
                    0_f32,
                    0_f32,
                    xpos,
                    ypos,
                    0_f32,
                    1_f32,
                    xpos + w,
                    ypos,
                    1_f32,
                    1_f32,
                    xpos,
                    ypos + h,
                    0_f32,
                    0_f32,
                    xpos + w,
                    ypos,
                    1_f32,
                    1_f32,
                    xpos + w,
                    ypos + h,
                    1_f32,
                    0_f32,
                ];

                ctex.texture.bind();
                self.text_vbo.update_data(&vertices);
                self.text_vbo.unbind();
                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                gl_panic!();

                _pos.x += (ctex.advance >> 6) as f32 * scale;
            }
            
            self.text_vao.unbind();
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            gl_panic!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
