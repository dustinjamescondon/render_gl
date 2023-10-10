use gl;


pub struct FrameBuffer {
    pub fbo: gl::types::GLuint,
}

#[allow(dead_code)]
impl FrameBuffer {
	pub fn new() -> FrameBuffer {
		let mut fbo: gl::types::GLuint = 0;
		unsafe {
			gl::GenFramebuffers(1, &mut fbo);
		}
		FrameBuffer{
			fbo,
		}
	}
	
	pub fn bind(&self) {
		unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo); }
	}

    pub fn unbind(&self) {
		unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, 0); }
	}

	pub fn status(&self) -> bool {
		self.bind();
		unsafe {
			gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
		}
	}
}

pub fn save_currently_bound_framebuffer() -> gl::types::GLint {
	let mut current_fbo: gl::types::GLint = 0;
	unsafe {
	    gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut current_fbo);
	}
	current_fbo
}

pub fn restore_framebuffer(target: gl::types::GLint) {
	unsafe {
		gl::BindFramebuffer(gl::FRAMEBUFFER, target as u32);
	}
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteFramebuffers(1, &self.fbo);
	}
	self.fbo = 0;
    }
}


    
