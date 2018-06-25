use gl;
use gl::types::*;

use error::{GlResult, GlError};

pub fn create_vao() -> GlResult<GLuint> {
    unsafe {
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        if vao == 0 {
            return Err(GlError::VaoCreation);
        }
        Ok(vao)
    }
}

pub fn create_buffer() -> GlResult<GLuint> {
    unsafe {
        let mut b = 0;
        gl::GenBuffers(1, &mut b);
        if b == 0 {
            return Err(GlError::BufferCreation);
        }
        Ok(b)
    }
}
