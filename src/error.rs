use std::fmt;
use std::error;

pub type GlResult<T> = Result<T, GlError>;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum GlError {
    ProgramLinkage(Option<String>),
    ProgramCreation,
    ProgramValidation(Option<String>),
    ShaderCreation,
    ShaderCompilation(Option<String>),
    TextureCreation,
    BufferCreation,
    GL_INVALID_ENUM,
    GL_INVALID_VALUE,
    GL_INVALID_OPERATION,
    GL_OUT_OF_MEMORY,
    GL_UNKNOWN_ERROR
}

impl fmt::Display for GlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlError::ShaderCompilation(ref log) => {
                write!(f, "RenderError: Shader creation failed. Log:\n{}",
                    log.clone().unwrap_or("No log".to_string()))
            },
            GlError::ProgramLinkage(ref log) => {
                write!(f, "RenderError: Program linkage failed. Log:\n{}",
                    log.clone().unwrap_or("No log".to_string()))
            },
            GlError::ProgramValidation(ref log) => {
                write!(f, "RenderError: Program validation failed. Log:\n{}",
                    log.clone().unwrap_or("No log".to_string()))
            },
            _ => write!(f, "RenderError: {}", self.as_str())
        }
    }
}

impl error::Error for GlError {
    fn description(&self) -> &str {
        "A render error occured"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl GlError {
    fn as_str(&self) -> &str {
        match *self {
            GlError::ProgramLinkage(_) => "program linking failed",
            GlError::ProgramCreation => "program creation failed",
            GlError::ProgramValidation(_) => "program validation failed",
            GlError::ShaderCreation => "shader creation failed",
            GlError::ShaderCompilation(_) => "shader compilation failed",
            GlError::TextureCreation => "texture creation failed",
            GlError::BufferCreation => "buffer creation failed",
            GlError::GL_INVALID_ENUM => "GL_INVALID_ENUM",
            GlError::GL_INVALID_VALUE => "GL_INVALID_VALUE",
            GlError::GL_INVALID_OPERATION => "GL_INVALID_OPERATION",
            GlError::GL_OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            GlError::GL_UNKNOWN_ERROR => "GL_UNKNOWN_ERROR"
        }
    }
}
