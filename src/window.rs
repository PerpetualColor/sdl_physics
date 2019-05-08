use gl;
use crate::gl_render;
use crate::simulator::Simulator;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Character {
    pub texture_id: gl::types::GLuint,
    pub size_x: i32,
    pub size_y: i32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance: i64
}

pub struct WindowData {
    pub gl: gl::Gl,
    pub x_range: f32,
    pub y_range: f32,
    pub gridline_vbo: gl::types::GLuint,
    pub gridline_vao: gl::types::GLuint,
    pub gridline_program: gl_render::Program,
    pub gridline_vec: Vec<f32>,
    pub vector_vbo: [gl::types::GLuint; 3],
    pub vector_vao: gl::types::GLuint,
    pub vector_program: gl_render::Program,
    pub vector_coord_storage: Vec<f32>,
    pub vector_color_storage: Vec<f32>,
    pub vectors_require_update: bool,
    pub vec_range: f32,
    pub character_map: HashMap<u8, Character>, 

    pub simulator: Rc<RefCell<Simulator>>
}

impl WindowData {
    pub fn new(gl: &gl::Gl, x_range: f32, y_range: f32, sim: Rc<RefCell<Simulator>>) -> WindowData {
        WindowData {
            gl: gl.clone(),
            x_range: x_range,
            y_range: y_range,
            gridline_vbo: 0,
            gridline_program: gl_render::Program::blank_program(&gl),
            gridline_vao: 0,
            gridline_vec: Vec::new(),

            vector_vbo: [0, 0, 0],
            vector_program: gl_render::Program::blank_program(&gl),
            vector_vao: 0,
            vector_coord_storage: Vec::new(),
            vector_color_storage: Vec::new(),
            vec_range: 0.0,
            vectors_require_update: false,

            character_map: HashMap::new(),

            simulator: sim
        }
    }
}