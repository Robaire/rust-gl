use std::fs;
use gl;
use std::ffi::CString;

pub struct Shader {
    id: gl::types::GLuint
}

impl Shader {
    pub fn from_string(source: &str, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = create_shader(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_file(path: &str, kind: gl::types::GLenum) -> Result<Shader, String> {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(message) => panic!(format!("Shader creation failed: {}", message))
        };
        let id = create_shader(&source, kind)?;
        Ok(Shader { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        };
    }
}

pub struct Program {
    id: gl::types::GLuint,
    shader_ids: Vec<gl::types::GLuint>
}

impl Program {
    pub fn new() -> Program {
        let id = unsafe { gl::CreateProgram() };
        Program{ id, shader_ids: vec![] }
    }

    pub fn attach_shader(mut self, shader: &Shader) -> Program{
        unsafe { gl::AttachShader(self.id, shader.id); }
        self.shader_ids.push(shader.id);
        self
    }

    pub fn link(mut self) -> Result<Program, String>{

        // Attempt to link the program
        unsafe { gl::LinkProgram(self.id); }

        // Check if the program linked correctly
        let mut link_status: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut link_status); }

        if link_status == 1 {
            // Detach all shaders from the program after linking successfully
            for shader_id in self.shader_ids.drain(..) {
                unsafe { gl::DetachShader(self.id, shader_id); }
            }
            Ok(self)

        } else {
            // Get the length of the error log
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len); }

            // Read the error message
            let error = create_cstring(len);

            unsafe {
                gl::GetProgramInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            Err(error.to_string_lossy().into_owned())
        }
    }

    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        };
    }
}

/// Creates a shader object from source
fn create_shader(source: &str, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {

    // Create a shader object on the GPU
    let id = unsafe {
        gl::CreateShader(kind)
    };

    // Attempt to compile the shader
    unsafe {
        gl::ShaderSource(id, 1, &CString::new(source).unwrap().as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    };

    // Check if the shader compiled
    let mut compile_status: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_status);
    };

    // Return the shader if it compiled
    if compile_status == 1 {
        Ok(id)
    } else {
        // Get the length of the error log
        let mut len: gl::types::GLint = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }

        // Read the error
        let error = create_cstring(len);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        Err(error.to_string_lossy().into_owned())
    }
}

/// Creates an empty CString of length len
fn create_cstring(len: gl::types::GLint) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}