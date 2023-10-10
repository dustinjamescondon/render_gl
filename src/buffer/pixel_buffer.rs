//! The purpose of this module is to define a PixelBuffer that can 
//! retrieve texture data from openGL

use gl;
use super::*;

pub struct BufferTypePixel;
impl BufferType for BufferTypePixel {
    const BUFFER_TYPE: gl::types::GLuint = gl::PIXEL_PACK_BUFFER;
}

#[allow(dead_code)]
pub type PixelBuffer = Buffer<BufferTypePixel>;

#[allow(dead_code)]
impl PixelBuffer {
    pub fn get_texture_data(&self, size: usize) -> Vec<u32> {
        let mut data = vec!(0u32; size);
        unsafe {
            gl::GetBufferSubData(gl::TEXTURE_2D,
                0,
                ((size as usize) * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                data.as_mut_ptr() as *mut gl::types::GLvoid)
            }
            
        todo!();
    }
}