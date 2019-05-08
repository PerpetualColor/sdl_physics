use crate::gl_render;
use crate::simulator::{Simulator, Particle};
use crate::window;
use gl;
use freetype;

use std::ffi::CString;

impl Simulator {
    pub fn draw_particles(
        &self,
        buf_id: &gl::types::GLuint,
        vao: &gl::types::GLuint,
        buf_size: &mut usize,
        window_info: &window::WindowData,
    ) -> usize {
        let mut positions: Vec<f32> = Vec::with_capacity(self.particle_list.borrow().len());
        for p in self.particle_list.borrow().iter() {
            let pos = p.get_pos();
            positions.push(pos.x / window_info.x_range);
            positions.push(pos.y / window_info.y_range);
            positions.push(0.0);
        }

        if positions.len() > *buf_size {
            unsafe {
                window_info.gl.BindVertexArray(*vao);
                window_info.gl.BindBuffer(gl::ARRAY_BUFFER, *buf_id);
                window_info.gl.VertexAttribPointer(
                    0,
                    positions.len() as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );
                window_info.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (positions.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    positions.as_ptr() as *const gl::types::GLvoid,
                    gl::DYNAMIC_DRAW,
                );
            }
            *buf_size = positions.len();
        } else {
            unsafe {
                window_info.gl.BindVertexArray(*vao);
                window_info.gl.BindBuffer(gl::ARRAY_BUFFER, *buf_id);
                window_info.gl.VertexAttribPointer(
                    0,
                    positions.len() as gl::types::GLint,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );
            }
            unsafe {
                window_info.gl.BindBuffer(gl::ARRAY_BUFFER, *buf_id);
                window_info.gl.BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (positions.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    positions.as_ptr() as *const gl::types::GLvoid,
                );
            }
        }
        if positions.len() != *buf_size {
            *buf_size = positions.len() / 3;
        }

        return positions.len() / 3;
    }

    pub fn draw_vectors(window_info: &mut window::WindowData) {
        if window_info.vectors_require_update {
            window_info.vector_program = gl_render::Program::blank_program(&window_info.gl);
            window_info.vector_color_storage = Vec::new();
            window_info.vector_coord_storage = Vec::new();
            window_info.vector_vao = 0;
            window_info.vector_vbo = [0, 0, 0];
            window_info.vectors_require_update = false;
        }
        if window_info.vector_program.id() == 0 {
            let vertex_shader = gl_render::Shader::from_vertex_source(
                &window_info.gl,
                &CString::new(include_str!("assets/shaders/vector.vert")).unwrap(),
            )
            .unwrap();
            let fragment_shader = gl_render::Shader::from_frag_source(
                &window_info.gl,
                &CString::new(include_str!("assets/shaders/vector.frag")).unwrap(),
            )
            .unwrap();

            // vectors
            let mut max_mag = 0.0_f32;
            let simulator = window_info.simulator.borrow_mut();
            window_info.vector_program = gl_render::Program::from_shaders(
                &window_info.gl,
                &mut [vertex_shader, fragment_shader],
            )
            .unwrap();
            for c in (((-window_info.x_range) as isize)..(window_info.y_range as isize)).step_by(2)
            {
                for r in
                    (((-window_info.y_range) as isize)..(window_info.y_range as isize)).step_by(2)
                {
                    let force = Simulator::acceleration_for(
                        &simulator,
                        &Particle::new(c as f32, r as f32, 0.0, 0.0),
                    );
                    let theta = force.y.atan2(force.x);
                    let mag = (force.x.powi(2) + force.y.powi(2)).sqrt() as f32;
                    if mag > max_mag {
                        max_mag = mag;
                    }

                    // starts at a point
                    window_info
                        .vector_coord_storage
                        .push(c as f32 / window_info.x_range);
                    window_info
                        .vector_coord_storage
                        .push(r as f32 / window_info.y_range);
                    window_info.vector_coord_storage.push(0.0);
                    window_info.vector_color_storage.push(mag.abs());

                    // goes at some angle
                    // radius always == 1
                    if mag != 0.0 {
                        window_info.vector_coord_storage.push(
                            (theta.cos() as f32 * 0.8 / window_info.x_range)
                                + (c as f32 / window_info.x_range),
                        );
                        window_info.vector_coord_storage.push(
                            (theta.sin() as f32 * 0.8 / window_info.y_range)
                                + (r as f32 / window_info.y_range),
                        );
                    } else {
                        window_info
                            .vector_coord_storage
                            .push(c as f32 / window_info.x_range);
                        window_info
                            .vector_coord_storage
                            .push(r as f32 / window_info.y_range);
                    }
                    window_info.vector_coord_storage.push(0.0);

                    window_info.vector_color_storage.push(mag.abs());
                }
            }
            window_info.vec_range = max_mag;

            unsafe {
                window_info.gl.GenBuffers(3, &mut window_info.vector_vbo[0]);
                window_info
                    .gl
                    .BindBuffer(gl::ARRAY_BUFFER, window_info.vector_vbo[0]);
                window_info.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (window_info.vector_coord_storage.len() * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    window_info.vector_coord_storage.as_ptr() as *const gl::types::GLvoid,
                    gl::DYNAMIC_DRAW,
                );
                window_info
                    .gl
                    .BindBuffer(gl::ARRAY_BUFFER, window_info.vector_vbo[1]);
                window_info.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (window_info.vector_color_storage.len() * std::mem::size_of::<f32>())
                        as gl::types::GLsizeiptr,
                    window_info.vector_color_storage.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
                window_info
                    .gl
                    .GenVertexArrays(1, &mut window_info.vector_vao);
                window_info.gl.BindVertexArray(window_info.vector_vao);
                window_info
                    .gl
                    .BindBuffer(gl::ARRAY_BUFFER, window_info.vector_vbo[0]);
                window_info.gl.EnableVertexAttribArray(0);
                window_info.gl.VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    std::ptr::null(),
                );
                window_info
                    .gl
                    .BindBuffer(gl::ARRAY_BUFFER, window_info.vector_vbo[1]);
                window_info.gl.EnableVertexAttribArray(1);
                window_info.gl.VertexAttribPointer(
                    1,
                    1,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<f32>() as gl::types::GLint,
                    std::ptr::null(),
                );
                window_info.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
                window_info.gl.BindVertexArray(0);
            }
        }
        window_info.vector_program.set_used();
        let range_location = unsafe {
            window_info.gl.GetUniformLocation(
                window_info.vector_program.id(),
                (CString::new("in_range").unwrap()).as_ptr(),
            )
        };

        unsafe {
            window_info.gl.LineWidth(2.0);
            window_info
                .gl
                .Uniform1f(range_location, window_info.vec_range);
            window_info.gl.BindVertexArray(window_info.vector_vao);
            window_info.gl.DrawArrays(
                gl::LINES,
                0,
                (window_info.vector_coord_storage.len() / 3) as i32,
            );
        }
    }
}

pub fn draw_gridlines(window_info: &mut window::WindowData) {
    if window_info.gridline_program.id() == 0 {
        let vertex_shader = gl_render::Shader::from_vertex_source(
            &window_info.gl,
            &CString::new(include_str!("assets/shaders/line.vert")).unwrap(),
        )
        .unwrap();
        let fragment_shader = gl_render::Shader::from_frag_source(
            &window_info.gl,
            &CString::new(include_str!("assets/shaders/line.frag")).unwrap(),
        )
        .unwrap();

        // gridlines

        window_info.gridline_program = gl_render::Program::from_shaders(
            &window_info.gl,
            &mut [vertex_shader, fragment_shader],
        )
        .unwrap();
        for v in ((-window_info.x_range) as isize)..(window_info.x_range as isize) {
            window_info
                .gridline_vec
                .push((v as f32) / window_info.x_range);
            window_info.gridline_vec.push(1.0);
            window_info.gridline_vec.push(0.0);

            window_info
                .gridline_vec
                .push(v as f32 / window_info.x_range);
            window_info.gridline_vec.push(-1.0);
            window_info.gridline_vec.push(0.0);
        }
        for v in ((-window_info.y_range) as isize + 1)..(window_info.y_range as isize) {
            window_info.gridline_vec.push(-1.0);
            window_info
                .gridline_vec
                .push(v as f32 / window_info.y_range);
            window_info.gridline_vec.push(0.0);

            window_info.gridline_vec.push(1.0);
            window_info
                .gridline_vec
                .push((v as f32) / window_info.y_range);
            window_info.gridline_vec.push(0.0);
        }
        window_info.gridline_vec.extend(&[-1.0, 0.0, 0.0]);
        window_info.gridline_vec.extend(&[1.0, 0.0, 0.0]);
        window_info.gridline_vec.extend(&[0.0, -1.0, 0.0]);
        window_info.gridline_vec.extend(&[0.0, 1.0, 0.0]);

        unsafe {
            window_info.gl.GenBuffers(1, &mut window_info.gridline_vbo);
            window_info
                .gl
                .BindBuffer(gl::ARRAY_BUFFER, window_info.gridline_vbo);
            window_info.gl.BufferData(
                gl::ARRAY_BUFFER,
                (window_info.gridline_vec.len() * std::mem::size_of::<f32>())
                    as gl::types::GLsizeiptr,
                window_info.gridline_vec.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            window_info
                .gl
                .GenVertexArrays(1, &mut window_info.gridline_vao);
            window_info.gl.BindVertexArray(window_info.gridline_vao);
            window_info.gl.EnableVertexAttribArray(0);
            window_info.gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }
    window_info.gridline_program.set_used();
    let vertex_color_location = unsafe {
        window_info.gl.GetUniformLocation(
            window_info.gridline_program.id(),
            (CString::new("inColor").unwrap()).as_ptr(),
        )
    };

    unsafe {
        window_info
            .gl
            .Uniform4f(vertex_color_location, 0.5, 0.5, 0.5, 1.0);
        window_info.gl.LineWidth(1.0);
        window_info.gl.BindVertexArray(window_info.gridline_vao);
        window_info
            .gl
            .BindBuffer(gl::ARRAY_BUFFER, window_info.gridline_vbo);
        window_info.gl.DrawArrays(
            gl::LINES,
            0,
            (window_info.gridline_vec.len() / 3) as i32 - 4,
        );
        window_info
            .gl
            .Uniform4f(vertex_color_location, 0.0, 0.0, 0.0, 1.0);
        window_info.gl.LineWidth(3.0);
        window_info.gl.DrawArrays(
            gl::LINES,
            (window_info.gridline_vec.len() / 3) as i32 - 4,
            4,
        );
    }
}

pub fn render_text(
    gl: &gl::Gl,
    text: &str,
    font: &freetype::face::Face,
    program: &gl_render::Program,
    vao: &mut gl::types::GLuint,
    vbo: &mut gl::types::GLuint,
    window_info: &mut window::WindowData,
    mut x: f32,
    y: f32,
    scale: f32,
) {
    let glyphs = (&text).as_bytes();
    unsafe { gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1) };
    // initialization
    program.set_used();
    unsafe {
        // gl.Uniform3f(gl.GetUniformLocation(program.id(), CString::new("textColor").unwrap().as_ptr()), 0.0, 0.0, 0.0);
        // column-major
        // let projection = vec![0.0025, 0.0, 0.0, -1.0, /* 2 */ 0.0, 0.003333, 0.0, -1.0, /* 3 */ 0.0, 0.0, -1.0, 0.0, /* 4 */ 0.0, 0.0, 0.0, 1.0];
        // row-major
        let projection: Vec<f32> = vec![0.002500, 0.000000, 0.000000, 0.000000, 0.000000, 0.003333, 0.000000, 0.000000, 0.000000, 0.000000, -1.000000, 0.000000, -1.000000, -1.000000, 0.000000, 1.000000];
        
        gl.UniformMatrix4fv(gl.GetUniformLocation(program.id(), CString::new("projection").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr() as *mut f32);
        gl.ActiveTexture(gl::TEXTURE0);
        gl.BindVertexArray(*vao);
    }

    for glyph in glyphs {
        if !window_info.character_map.contains_key(&glyph) {
            font.load_char(*glyph as usize, freetype::face::LoadFlag::RENDER).unwrap();
            let g = font.glyph();
            let mut texture: gl::types::GLuint = 0;
            
            unsafe {
                gl.GenTextures(1, &mut texture);
                gl.BindTexture(gl::TEXTURE_2D, texture);
                gl.TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    g.bitmap().width(),
                    g.bitmap().rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    g.bitmap().buffer().as_ptr() as *const gl::types::GLvoid,
                );
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }
            window_info.character_map.insert(*glyph, window::Character {
                texture_id: texture,
                size_x: g.bitmap().width(),
                size_y: g.bitmap().rows(),
                bearing_x: g.bitmap_left(),
                bearing_y: g.bitmap_top(),
                advance: g.advance().x
            });
        }
        let c = window_info.character_map.get(&glyph).unwrap();

        let x_pos = x + (c.bearing_x as f32) * scale;
        let y_pos = y - (c.size_y - c.bearing_y) as f32 * scale;
        
        let w = c.size_x as f32 * scale;
        let h = c.size_y as f32 * scale;
        
        // let x_pos = 0.0;
        // let y_pos = 0.0;
        // let w = 50.0;
        // let h = 50.0;

        let vertices: Vec<f32> = vec![
            x_pos, y_pos + h, 0.0, 0.0,
            x_pos, y_pos, 0.0, 1.0,
            x_pos + w, y_pos, 1.0, 1.0,

            x_pos, y_pos + h, 0.0, 0.0,
            x_pos + w, y_pos, 1.0, 1.0,
            x_pos + w, y_pos + h, 1.0, 0.0
        ];

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, c.texture_id);
            gl.BindBuffer(gl::ARRAY_BUFFER, *vbo);
            gl.BufferSubData(gl::ARRAY_BUFFER, 0, (vertices.len() * std::mem::size_of::<f32>()) as isize, vertices.as_ptr() as *const gl::types::GLvoid);
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }
        x += (c.advance / 64 ) as f32 * scale;
    }
    unsafe {
        gl.BindVertexArray(0);
        gl.BindTexture(gl::TEXTURE_2D, 0);
    }
}
