use gl;
use sdl2::pixels::Color;
use std::os::raw::c_void;
use crate::glchk;

// Refer https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml
// and https://moderngl.readthedocs.io/en/latest/topics/texture_formats.html
pub trait TextureType {
	const INTERNAL_FORMAT: gl::types::GLuint;
    const TEXTURE_TYPE: gl::types::GLuint;
	const DATA_TYPE: gl::types::GLenum;
}

pub struct Texture<T>
where 
	T: TextureType
{
	pub id: gl::types::GLuint,
	pub width: gl::types::GLint,
	pub height: gl::types::GLint,
	pub target: gl::types::GLuint,
	pub attach_point: gl::types::GLenum,
	_hack: std::marker::PhantomData<T>,
}

pub type RGBTexture = Texture<TextureTypeRGB>;
pub type U32Texture = Texture<TextureTypeI32>;
pub type REDTexture = Texture<TextureTypeRed>;

pub struct TextureTypeRGB;
impl TextureType for TextureTypeRGB {
	const INTERNAL_FORMAT: gl::types::GLuint = gl::RGB4;
	const TEXTURE_TYPE: gl::types::GLuint = gl::RGB;
	const DATA_TYPE: gl::types::GLenum = gl::FLOAT;
}

pub struct TextureTypeI32;
impl TextureType for TextureTypeI32 {
	const INTERNAL_FORMAT: gl::types::GLuint = gl::R32I;
	const TEXTURE_TYPE: gl::types::GLuint = gl::RED_INTEGER;
	const DATA_TYPE: gl::types::GLenum = gl::INT;
}

pub struct TextureTypeRed;
impl TextureType for TextureTypeRed {
	const INTERNAL_FORMAT: gl::types::GLuint = gl::RED;
	const TEXTURE_TYPE: gl::types::GLuint = gl::RED;
	const DATA_TYPE: gl::types::GLenum = gl::UNSIGNED_BYTE;
}

pub struct TextureTypeF32;
impl TextureType for TextureTypeF32 {
	const INTERNAL_FORMAT: gl::types::GLuint = gl::R32F;
	const TEXTURE_TYPE: gl::types::GLuint = gl::RED;
	const DATA_TYPE: gl::types::GLenum = gl::FLOAT;
}

impl Clone for RGBTexture {
    fn clone(&self) -> Self {
        RGBTexture::new_from_data(
			self.get_pixel_data().as_mut_ptr() as *mut gl::types::GLvoid, 
			self.width as usize, self.height as usize
		)
    }
}

#[allow(dead_code)]
impl<T> Texture<T> 
where T: TextureType {	
	/// This allocates a texture on the video card of the given size 
	/// without any data attached to it. This texture can be bound to 
	/// a framebuffer and be written to by a shader
	pub fn new(width: i32, height: i32) -> Self {
		let target = gl::TEXTURE_2D;
		
		let id = create_and_set_gl_parameters();
		unsafe {
			gl::TexImage2D(target,
				0,
				T::INTERNAL_FORMAT as i32,
				width,
				height,
				0,
				T::TEXTURE_TYPE,
				T::DATA_TYPE,
				std::ptr::null());
			}
			
			Texture {
			id,
			width,
			height,
			target,
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
		let target = gl::TEXTURE_2D;
		
		let id = create_and_set_gl_parameters();
		unsafe {
			gl::TexImage2D(target, 
				0,
				T::INTERNAL_FORMAT as i32,
				width as i32, 
				height as i32,
				0,
				T::TEXTURE_TYPE,
				T::DATA_TYPE,
				data as *const c_void);
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
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}

	pub fn get_int_pixel_data(&self) -> Vec<i32>
	{
		self.bind();
		let mut data = vec![0i32; (self.width * self.height) as usize];
		unsafe {
			glchk!(
			gl::GetTexImage(gl::TEXTURE_RECTANGLE,
				0,
				T::TEXTURE_TYPE,
				T::DATA_TYPE,
				data.as_mut_ptr() as *mut gl::types::GLvoid,
			);
		);
		}
		
		data
	}

	pub fn get_pixel_data(&self) -> Vec<f32>
	{
		self.bind();
		let mut data = vec![0.0f32; (self.width * self.height * 3) as usize];
		unsafe {
			glchk!(
			gl::GetTexImage(gl::TEXTURE_RECTANGLE,
				0,
				T::TEXTURE_TYPE,
				T::DATA_TYPE,
				data.as_mut_ptr() as *mut gl::types::GLvoid,
			););
		}
		
		data
	}
	
	pub fn get_pixel_rgb_data(&self) -> Vec<Color>
	{
		self.bind();
		let mut data = vec![0.0f32; (self.width * self.height * 3) as usize];
		unsafe {
			glchk!(
			gl::GetTexImage(gl::TEXTURE_RECTANGLE,
				0,
				T::TEXTURE_TYPE,
				T::DATA_TYPE,
				data.as_mut_ptr() as *mut gl::types::GLvoid,
			););
		}
		
		data.chunks(3)
			.map(|arr| normalized_to_u8_color(arr[0], arr[1], arr[2])).collect()
	}

	pub fn attach_to_fbo(&mut self, attach_point: gl::types::GLenum) {
		unsafe {
			gl::FramebufferTexture2D(gl::FRAMEBUFFER, attach_point, self.target, self.id, 0);
		}
		self.attach_point = attach_point;
	}
	
	pub fn attach_to_unit(&mut self, tex_unit: gl::types::GLuint) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
		self.attach_point = gl::TEXTURE_2D + tex_unit;
	}
}

fn create_and_set_gl_parameters() -> gl::types::GLuint  {
	let mut id: gl::types::GLuint = 0;
	let target = gl::TEXTURE_2D;
	
	unsafe {
		gl::GenTextures(1, &mut id);
		gl::BindTexture(target, id);
		gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
		gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
		gl::TexParameteri(target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
		gl::TexParameteri(target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
	}		
	id
}

fn normalized_to_u8_color(r: f32, g: f32, b: f32) -> Color {
	let max_u8 = u8::MAX as f32;
	Color::RGB(
		(r * max_u8) as u8,
	 	(g * max_u8) as u8,
	  	(b * max_u8) as u8)
}

impl<T> Drop for Texture<T> 
where T:TextureType,
{
	fn drop(&mut self) {
		unsafe { gl::DeleteTextures(1, &self.id); }
	}
}
