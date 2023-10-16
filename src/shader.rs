use std::ffi::{CStr, CString};
use nalgebra::{Matrix4};

pub struct Program {
    id: gl::types::GLuint,
}

#[allow(dead_code)]
impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }
        
        Ok(Program { id: program_id })
    }
    
    pub fn from_src(vert_src: &str, frag_src: &str) -> Result<Program, String> {
        let vertex_shader = Shader::from_vert_source(&CString::new(vert_src).unwrap());
        if let Err(err) = vertex_shader {
            return Err(format!("{}{}", "Error compiling vertex shader", &err));
        }
        
        let fragment_shader = Shader::from_frag_source(&CString::new(frag_src).unwrap());
        if let Err(err) = fragment_shader {
            return Err(format!("{}{}", "Error compiling fragment shader", &err));
        }
        Program::from_shaders(&[vertex_shader.unwrap(), fragment_shader.unwrap()])
    }
    
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    
    pub fn get_uniform_location(&self, name: String) -> i32 {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, c_str.as_ptr())
        }
    }
    
    pub fn set_uniform_m4f(&self, name: String, matrix: &Matrix4<f32>) {
        
        let loc = self.get_uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(loc, 1, 0_u8, matrix.as_ptr());
        }
    }

    pub fn set_uniform_glm_m4(&self, name: String, matrix: &nalgebra_glm::Mat4) {
        
        let loc = self.get_uniform_location(name);
        unsafe {
            gl::UniformMatrix4fv(loc, 1, 0_u8, matrix.as_ptr());
        }
    }
    
    

    pub fn set_bool(&self, name: String, value: bool) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1i(location, value as i32);
        }
    }
    
    pub fn set_int(&self, name: String, value: i32) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1i(location, value as i32);
        }
    }
    
    pub fn set_float(&self, name: String, value: f32) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1f(location, value as f32);
        }
    }

    pub fn set_3float(&self, name: String, value: [f32;3]) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform3f(location, value[0], value[1], value[2]);
        }
    }

    pub fn set_double(&self, name: String, value: f64) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1d(location, value);
        }
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
