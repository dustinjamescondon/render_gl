mod pixel_buffer;

use gl;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct Buffer<B>
where
    B: BufferType,
{
    vbo: gl::types::GLuint,
    _hack: std::marker::PhantomData<B>, // need this otherwise the compiler complains about the unused 'B'
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}

pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

impl<B> Buffer<B>
where
    B: BufferType,
{
    pub fn new() -> Buffer<B> {
        use std::marker::PhantomData;

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            _hack: PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    #[allow(dead_code)]
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    #[allow(dead_code)]
    pub fn dynamic_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    /// TODO not sure this is working correctly
    #[allow(dead_code)]
    pub fn update_data<T>(&self, data:&[T]) {
        self.bind();
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER, 
                0,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid);
        }
    }
}

impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
