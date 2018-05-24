use gl;
use gl::types::*;

use error::{GlResult, GlError};

macro_rules! get_info_log {
    ($get_attr:path, $get_log:path, $gl_id:expr) => {{
        let mut log_length_glint: GLint = 0;
        $get_attr($gl_id, gl::INFO_LOG_LENGTH, &mut log_length_glint);
        let log_length = log_length_glint as usize;
        if log_length == 0 {
            None
        } else {
            let mut raw_log = Vec::<u8>::with_capacity(log_length);
            $get_log($gl_id, log_length as GLsizei,
                0 as *mut GLsizei, raw_log.as_mut_ptr() as *mut GLchar);
            raw_log.set_len(log_length);
            let log = String::from_utf8(raw_log)
                .expect("OpenGL returned invalid utf8 in a program info log");
            Some(log)
        }
    }}
}

pub fn create_program() -> GlResult<GLuint> {
    let gl_id = unsafe { gl::CreateProgram() };
    if gl_id == 0 {
        Err(GlError::ProgramCreation)
    } else {
        Ok(gl_id)
    }
}

pub fn create_shader(kind: GLenum, source: &str) -> GlResult<GLuint> {
    unsafe {
        let gl_id = gl::CreateShader(kind as _);
        if gl_id == 0 {
            return Err(GlError::ShaderCreation);
        }
        gl::ShaderSource(
            gl_id,
            1,
            &(source.as_ptr() as *const _),
            &(source.len() as _)
        );
        gl::CompileShader(gl_id);

        let mut status = 0;
        gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut status);
        if status != 1 {
            Err(GlError::ShaderCompilation(
                get_info_log!(gl::GetShaderiv, gl::GetShaderInfoLog, gl_id)
            ))
        } else {
            Ok(gl_id)
        }
    }
}

pub fn get_link_status(program_id: GLuint) -> GlResult<()> {
    let mut link_status = gl::FALSE as i32;
    unsafe {
        gl::GetProgramiv(
            program_id,
            gl::LINK_STATUS,
            &mut link_status as *mut _
        );
        if link_status != gl::TRUE as i32 {
            Err(GlError::ProgramLinkage(
                get_info_log!(gl::GetProgramiv, gl::GetProgramInfoLog, program_id)
            ))
        } else {
            Ok(())
        }
    }
}

pub fn create_basic_program(vertex_source: &str, fragment_source: &str) -> GlResult<GLuint> {
    let program = create_program()?;
    let vertex_shader = create_shader(gl::VERTEX_SHADER, vertex_source)?;
    let fragment_shader = create_shader(gl::FRAGMENT_SHADER, fragment_source)?;
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        // n.b., you must do attribute binding before linking!
        // (but you can always use the attribute location getter)
        gl::LinkProgram(program);
        get_link_status(program)?;
        // we have to detach the shaders before the shader objects will be freed
        gl::DetachShader(program, vertex_shader);
        gl::DetachShader(program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    Ok(program)
}
