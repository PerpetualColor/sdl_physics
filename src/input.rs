use crate::gl_render;
use crate::gl_render::Program;
use gl;
use std::ffi::CString;

pub struct Button {
    gl: gl::Gl,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub program: Program,
    pub vao_id: gl::types::GLuint,
    pub vbo_id: gl::types::GLuint,
    pub coordinates: Vec<f32>,
    pub onclick: String
}

impl Button {
    pub fn new(gl: &gl::Gl, x: f32, y: f32, width: f32, height: f32, onclick: String) -> Button {
        Button {
            gl: gl.clone(),
            x: x,
            y: y,
            width: width,
            height: height,
            program: Program::blank_program(&gl),
            vao_id: 0,
            vbo_id: 0,
            coordinates: Vec::new(),
            onclick: onclick,
        }
    }

    pub fn onclick(&self) -> &str {
        &self.onclick
    }

    pub fn draw(&mut self) {
        if self.program.id() == 0 {
            let vertex_shader = gl_render::Shader::from_vertex_source(
                &self.gl,
                &CString::new(include_str!("assets/shaders/line.vert")).unwrap(),
            )
            .unwrap();
            let fragment_shader = gl_render::Shader::from_frag_source(
                &self.gl,
                &CString::new(include_str!("assets/shaders/line.frag")).unwrap(),
            )
            .unwrap();

            // gridlines
            self.program =
                gl_render::Program::from_shaders(&self.gl, &[vertex_shader, fragment_shader]).unwrap();
            // TODO: Generate coordinates here
            // top left
            self.coordinates.push(self.x);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);
            // top right
            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);
            // bottom right
            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);
            // bottom left
            self.coordinates.push(self.x);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);

            // outline
            self.coordinates.push(self.x);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);
            // top right
            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);


            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);
            // bottom right
            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);

            self.coordinates.push(self.x + self.width);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);
            // bottom left
            self.coordinates.push(self.x);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);

            self.coordinates.push(self.x);
            self.coordinates.push(self.y - self.height);
            self.coordinates.push(0.0);

            self.coordinates.push(self.x);
            self.coordinates.push(self.y);
            self.coordinates.push(0.0);


            unsafe {
                self.gl.GenBuffers(1, &mut self.vbo_id);
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (self.coordinates.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    self.coordinates.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                self.gl.GenVertexArrays(1, &mut self.vao_id);
                self.gl.BindVertexArray(self.vao_id);
                self.gl.EnableVertexAttribArray(0);
                self.gl.VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );
            }
        }
        self.program.set_used();
        let vertex_color_location = unsafe {
            self.gl.GetUniformLocation(
                self.program.id(),
                (CString::new("inColor").unwrap()).as_ptr(),
            )
        };
        unsafe {
            self.gl.Uniform4f(vertex_color_location, 0.95, 0.95, 0.95, 0.8);
            self.gl.LineWidth(1.0);
            self.gl.BindVertexArray(self.vao_id);
            self.gl.DrawArrays(gl::TRIANGLE_FAN, 0, (self.coordinates.len() / 3) as i32 - 8);
            self.gl.Uniform4f(vertex_color_location, 0.1, 0.1, 0.1, 1.0);
            self.gl.DrawArrays(gl::LINES, 4, 8);
        }
    }
}
