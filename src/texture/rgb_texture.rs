use crate::glchk;
use gl;
use std::os::raw::c_void;

use super::{ColorF32, ColorU8};

// Refer https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml
// and https://moderngl.readthedocs.io/en/latest/topics/texture_formats.html
pub trait TextureType<TData> {
    const INTERNAL_FORMAT: gl::types::GLuint;
    const TEXTURE_TYPE: gl::types::GLuint;
    const DATA_TYPE: gl::types::GLenum;
    const TARGET: gl::types::GLenum;
    const TEXTURE_WRAP_S: gl::types::GLenum;
    const TEXTURE_WRAP_T: gl::types::GLenum;
    const TEXTURE_MIN_FILTER: gl::types::GLenum;
    const TEXTURE_MAG_FILTER: gl::types::GLenum;
}

pub struct Texture<TTex,TData>
where
    TTex: TextureType<TData>,
	TData: Default + Clone
{
    pub id: gl::types::GLuint,
    pub width: gl::types::GLint,
    pub height: gl::types::GLint,
    pub target: gl::types::GLuint,
    pub attach_point: gl::types::GLenum,
    _hack: std::marker::PhantomData<(TTex,TData)>,
}

pub type RGBTexture = Texture<TextureTypeRGB, [f32;3]>; 
pub type U8RGBTexture = Texture<TextureTypeU8RGB, [u8;3]>;
pub type U8RGBATexture = Texture<TextureTypeU8RGBA, [u8;4]>;
pub type F32Texture = Texture<TextureTypeRGB, f32>;
pub type I32Texture = Texture<TextureTypeI32, i32>;
pub type REDTexture = Texture<TextureTypeRed, u8>;

pub struct TextureTypeRGB;
impl TextureType<[f32;3]> for TextureTypeRGB {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::RGB4;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RGB;
    const DATA_TYPE: gl::types::GLenum = gl::FLOAT;
    /// Is texture rectangle just b/c the only place I used this was 
    /// in a shader for the mandelbrot fractal, where I want to access
    /// things by pixel instead of normalized coorindates
    const TARGET: gl::types::GLenum = gl::TEXTURE_RECTANGLE;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

pub struct TextureTypeU8RGB;
impl TextureType<[u8;3]> for TextureTypeU8RGB {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::RGBA;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RGBA;
    const DATA_TYPE: gl::types::GLenum = gl::UNSIGNED_BYTE;
    const TARGET: gl::types::GLenum = gl::TEXTURE_2D;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::REPEAT;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::REPEAT;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR_MIPMAP_LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

pub struct TextureTypeU8RGBA;
impl TextureType<[u8;4]> for TextureTypeU8RGBA {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::RGBA;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RGBA;
    const DATA_TYPE: gl::types::GLenum = gl::UNSIGNED_BYTE;
    const TARGET: gl::types::GLenum = gl::TEXTURE_2D;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::REPEAT;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::REPEAT;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR_MIPMAP_LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

pub struct TextureTypeI32;
impl TextureType<i32> for TextureTypeI32 {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::R32I;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RED_INTEGER;
    const DATA_TYPE: gl::types::GLenum = gl::INT;
    const TARGET: gl::types::GLenum = gl::TEXTURE_RECTANGLE;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

pub struct TextureTypeRed;
impl TextureType<u8> for TextureTypeRed {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::RED;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RED;
    const DATA_TYPE: gl::types::GLenum = gl::UNSIGNED_BYTE;
    const TARGET: gl::types::GLenum = gl::TEXTURE_2D;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

pub struct TextureTypeF32;
impl TextureType<f32> for TextureTypeF32 {
    const INTERNAL_FORMAT: gl::types::GLuint = gl::R32F;
    const TEXTURE_TYPE: gl::types::GLuint = gl::RED;
    const DATA_TYPE: gl::types::GLenum = gl::FLOAT;
    const TARGET: gl::types::GLenum = gl::TEXTURE_2D;
    const TEXTURE_WRAP_S: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_WRAP_T: gl::types::GLenum = gl::CLAMP_TO_EDGE;
    const TEXTURE_MIN_FILTER: gl::types::GLenum = gl::LINEAR;
    const TEXTURE_MAG_FILTER: gl::types::GLenum = gl::LINEAR;
}

impl Clone for RGBTexture {
    fn clone(&self) -> Self {
        RGBTexture::new_from_data(
            self.get_pixel_data().as_mut_ptr() as *mut gl::types::GLvoid,
            self.width as usize,
            self.height as usize,
        )
    }
}

#[allow(dead_code)]
impl<TTex,TData> Texture<TTex,TData>
where
    TTex: TextureType<TData>,
	TData: Default + Clone
{
    /// This allocates a texture on the video card of the given size
    /// without any data attached to it. This texture can be bound to
    /// a framebuffer and be written to by a shader
    pub fn new(width: i32, height: i32) -> Self {
        let id = Self::create_and_set_gl_parameters();
        unsafe {
            gl::TexImage2D(
                TTex::TARGET,
                0,
                TTex::INTERNAL_FORMAT as i32,
                width,
                height,
                0,
                TTex::TEXTURE_TYPE,
                TTex::DATA_TYPE,
                std::ptr::null(),
            );
        }

        Texture {
            id,
            width,
            height,
            target: TTex::TARGET,
            attach_point: 0,
            _hack: std::marker::PhantomData,
        }
    }

    /// This allocates a texture on the video card of the given size
    /// containing the data.
    ///
    /// TODO maybe having a void pointer be necessary for this interface is a bad idea...
    /// that is, maybe we can make specific functions for passing in color data or u32 data
    /// instead of void*
    pub fn new_from_data(data: *const std::ffi::c_void, width: usize, height: usize) -> Self {
        let target = TTex::TARGET;

        let id = Self::create_and_set_gl_parameters();
        unsafe {
            gl::TexImage2D(
                target,
                0,
                TTex::INTERNAL_FORMAT as i32,
                width as i32,
                height as i32,
                0,
                TTex::TEXTURE_TYPE,
                TTex::DATA_TYPE,
                data as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture {
            id,
            width: width as i32,
            height: height as i32,
            target,
            attach_point: 0,
            _hack: std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(TTex::TARGET, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(TTex::TARGET, 0);
        }
    }

    pub fn get_pixel_data(&self) -> Vec<TData> {
        self.bind();
        let mut data = vec![TData::default(); (self.width * self.height) as usize];
        unsafe {
            glchk!(
                gl::GetTexImage(TTex::TARGET,
                    0,
                    TTex::TEXTURE_TYPE,
                    TTex::DATA_TYPE,
                    data.as_mut_ptr() as *mut gl::types::GLvoid,
                );
            );
        }

        data
    }

    pub fn attach_to_fbo(&mut self, attach_point: gl::types::GLenum) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attach_point, self.target, self.id, 0);
        }
        self.attach_point = attach_point;
    }

    pub fn attach_to_unit(&self, tex_unit: gl::types::GLuint) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
            gl::BindTexture(TTex::TARGET, self.id);
        }
        //self.attach_point = gl::TEXTURE0 + tex_unit;
    }

    /// TODO impl some mech for this to be cusomized per texture, because
    /// for instance a character texture needs specific settings
    fn create_and_set_gl_parameters() -> gl::types::GLuint {
	let mut id: gl::types::GLuint = 0;
	let target = TTex::TARGET;
	
	unsafe {
	    gl::GenTextures(1, &mut id);
	    gl::BindTexture(target, id);
	    gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, TTex::TEXTURE_MIN_FILTER as i32);
	    gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, TTex::TEXTURE_MAG_FILTER as i32);
	    gl::TexParameteri(target, gl::TEXTURE_WRAP_S, TTex::TEXTURE_WRAP_S as i32);
	    gl::TexParameteri(target, gl::TEXTURE_WRAP_T, TTex::TEXTURE_WRAP_T as i32);
	}
	id
    }
}

impl<TTex,TData> Drop for Texture<TTex,TData>
where
    TTex: TextureType<TData>,
	TData: Default + Clone
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
