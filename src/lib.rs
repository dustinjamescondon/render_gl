pub mod buffer;
pub mod framebuffer;
mod shader;
pub mod text;
pub mod texture;
pub mod camera;

pub use buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
pub use framebuffer::*;
pub use shader::{Program, Shader};
pub use texture::*;

#[macro_export]
macro_rules! gl_panic {
    () => {
        unsafe {
            if gl::GetError() != gl::NO_ERROR {
                panic!("OpenGL error!");
            }
        }
    };
}

#[macro_export]
macro_rules! glchk {
    ($($s:stmt;)*) => {
        $(
            $s
            if cfg!(debug_assertions) {
                let err = gl::GetError();
                if err != gl::NO_ERROR {
                    let err_str = match err {
                        gl::INVALID_ENUM => "GL_INVALID_ENUM",
                        gl::INVALID_VALUE => "GL_INVALID_VALUE",
                        gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
                        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                        gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                        _ => "unknown error"
                    };
                    println!("{}:{} - {} caused {}",
                             file!(),
                             line!(),
                             stringify!($s),
                             err_str);
                }
            }
        )*
    }
}

/*
pub fn setup_error_callback() {
    unsafe {
        gl::Enable(gl::DEBUG_CALLBACK_FUNCTION);
        gl::DebugMessageCallback(message_callback, 0)
    }
}

fn message_callback(
    source: gl::types::GLenum,
    type_: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    length: isize,
    message: *const gl::types::GLchar,
    userParam: *const gl::types::GLvoid)
    {

    }*/
