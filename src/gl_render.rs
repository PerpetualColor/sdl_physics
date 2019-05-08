use gl;
use std::ffi::CStr;

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint
}

pub struct Program {
    pub gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn blank_program(gl: &gl::Gl) -> Program {
        Program {
            gl: gl.clone(),
            id: 0
        }
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl.CreateProgram() };
        // add and drop shaders
        for s in shaders {
            unsafe {
                gl.AttachShader(id, s.id);
            }
        }
        unsafe {
            gl.LinkProgram(id);
        }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl.GetProgramiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut error: Vec<u8> = Vec::with_capacity(len as usize + 1);
            error.extend([b' '].iter().cycle().take(len as usize));

            unsafe {
                gl.GetProgramInfoLog(
                    id, 
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(String::from_utf8(error).unwrap());
        }
        Ok(Program { gl: gl.clone(), id: id })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

impl Shader {

    fn from_source(gl: &gl::Gl, src: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        Ok(Shader {
            gl: gl.clone(),
            id: shader_from_source(&gl, src, kind).unwrap()
        })
    }

    pub fn from_vertex_source(gl: &gl::Gl, src: &CStr) -> Result<Shader, String> {
        Ok(Shader::from_source(gl, src, gl::VERTEX_SHADER).unwrap())
    }

    pub fn from_frag_source(gl: &gl::Gl, src: &CStr) -> Result<Shader, String> {
        Ok(Shader::from_source(gl, src, gl::FRAGMENT_SHADER).unwrap())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(gl: &gl::Gl, src: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id: gl::types::GLuint = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut error: Vec<u8> = Vec::with_capacity(len as usize + 1);
        error.extend([b' '].iter().cycle().take(len as usize));

        unsafe {
            gl.GetShaderInfoLog(
                id, 
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(String::from_utf8(error).unwrap());

    }

    Ok(id)
}