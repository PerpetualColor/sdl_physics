use freetype;
use gl;
use sdl2;
use std::cell::RefCell;
use std::cmp;
use std::ffi::CString;
use std::rc::Rc;
use std::f32::consts::PI;

mod gl_draw;
mod gl_render;
mod input;
mod simulator;
mod window;

use simulator::util::PhysVector;
use simulator::Simulator;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Physics Simulator", 500, 500)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let (wsize_x, wsize_y) = window.size();

    let (mut x, mut y): (i32, i32) = (wsize_x as i32, wsize_y as i32);
    let mut width: i32 = cmp::min(x as i32, y as i32);
    let mut x_offset = (x - width) / 2;
    let mut y_offset = (y - width) / 2;

    let _gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync).unwrap();
    // create shaders and program

    let vertex_shader = gl_render::Shader::from_vertex_source(
        &gl,
        &CString::new(include_str!("assets/shaders/triangle.vert")).unwrap(),
    )
    .unwrap();
    let fragment_shader = gl_render::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("assets/shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    let program = gl_render::Program::from_shaders(&gl, &[vertex_shader, fragment_shader]).unwrap();

    // create geometry
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    // write buffer into gl buffer
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_DRAW,
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    // make corresponding element buffer object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    unsafe {
        gl.Viewport(0, 0, 500, 500);
        gl.ClearColor(1.0, 1.0, 1.0, 1.0);
        gl.PointSize(7.0);
    }

    // setup simulator
    let sim = Rc::new(RefCell::new(Simulator::new(1.0 / 120.0)));

    for i in -20..20 {
        for j in -20..20 {
            sim.borrow_mut().add_particle(i as f32, j as f32, 0.0, 0.0);
        }
    }

    let mut current_buf_size = vertices.len();
    let mut pause = true;

    let mut window_info = window::WindowData::new(&gl, 30.0, 30.0, sim.clone());
    let play_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.95,
        0.16,
        0.06,
        "play".to_string(),
    )));
    let pause_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.87,
        0.16,
        0.06,
        "pause".to_string(),
    )));
    let clear_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.79,
        0.16,
        0.06,
        "clear".to_string(),
    )));
    let grid_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.71,
        0.16,
        0.06,
        "create_grid".to_string(),
    )));
    let sine_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95 + 0.20,
        0.71,
        0.16,
        0.06,
        "create_sine".to_string(),
    )));

    let gravity_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.63,
        0.16,
        0.06,
        "force_Gravity".to_string(),
    )));
    let gravity_resistive_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.55,
        0.16 * 3.0,
        0.06,
        "force_GravityResistive".to_string(),
    )));
    let butterfly_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.47,
        0.16 * 2.0,
        0.06,
        "force_Butterfly".to_string(),
    )));
    let windows_xp_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.39,
        0.16 * 2.0,
        0.06,
        "force_WindowsXP".to_string(),
    )));
    let logistic_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.31,
        0.16,
        0.06,
        "force_Logistic".to_string(),
    )));
    let inverse_square_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.23,
        0.32,
        0.06,
        "force_InverseSquare".to_string(),
    )));
    let parallel_electric_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.15,
        0.32,
        0.06,
        "force_ParallelElectric".to_string(),
    )));
    let harmonic_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        0.07,
        0.32,
        0.06,
        "force_Harmonic".to_string(),
    )));
    let no_force_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95,
        -0.01,
        0.32,
        0.06,
        "force_NoForce".to_string(),
    )));
    let bounce_button = Rc::new(RefCell::new(input::Button::new(
        &gl,
        -0.95 + 0.20,
        0.87,
        0.32,
        0.06,
        "toggle_bounce".to_string(),
    )));
    let mut bounce_string = String::from("Enable Bounce");

    let mut buttons_vec: Vec<Rc<RefCell<input::Button>>> = Vec::new();
    buttons_vec.push(play_button);
    buttons_vec.push(pause_button);
    buttons_vec.push(clear_button);
    buttons_vec.push(grid_button);
    buttons_vec.push(gravity_button);
    buttons_vec.push(gravity_resistive_button);
    buttons_vec.push(butterfly_button);
    buttons_vec.push(windows_xp_button);
    buttons_vec.push(logistic_button);
    buttons_vec.push(bounce_button);
    buttons_vec.push(inverse_square_button);
    buttons_vec.push(parallel_electric_button);
    buttons_vec.push(harmonic_button);
    buttons_vec.push(sine_button);
    buttons_vec.push(no_force_button);

    // setup freetype

    let ft_lib = freetype::Library::init().unwrap();
    let ft_face = ft_lib.new_face("./Ubuntu-R.ttf", 0).unwrap();
    ft_face.set_char_size(40 * 64 * 2, 0, 50, 0).unwrap();

    let vertex_shader = gl_render::Shader::from_vertex_source(
        &gl,
        &CString::new(include_str!("assets/shaders/glyph.vert")).unwrap(),
    )
    .unwrap();
    let fragment_shader = gl_render::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("assets/shaders/glyph.frag")).unwrap(),
    )
    .unwrap();
    let ft_program =
        gl_render::Program::from_shaders(&gl, &[vertex_shader, fragment_shader]).unwrap();

    let mut ft_vao: gl::types::GLuint = 0;
    let mut ft_vbo: gl::types::GLuint = 0;

    let mut event_pump = sdl.event_pump().unwrap();
    let mut creating_particle = false;
    let mut particle_start = PhysVector { x: 0.0, y: 0.0 };
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    sdl2::event::WindowEvent::Resized {
                        0: found_x,
                        1: found_y,
                    } => {
                        x = found_x;
                        y = found_y;
                        width = cmp::min(x, y);
                        x_offset = (x - width) / 2;
                        y_offset = (y - width) / 2;
                        unsafe {
                            gl.Viewport(x_offset, y_offset, width, width);
                        }
                    }
                    _ => {}
                },
                sdl2::event::Event::MouseButtonDown {
                    timestamp: _ts,
                    window_id: _wi,
                    which: _wh,
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    clicks: _cl,
                    x,
                    y,
                } => {
                    let c_x = ((x - x_offset) as f32 * 2.0 / width as f32) - 1.0;
                    let c_y = (((y - y_offset) as f32 * 2.0 / width as f32) - 1.0) * -1.0;

                    let mut button_found = false;

                    for b in &buttons_vec {
                        let b_x = b.borrow().x;
                        let b_y = b.borrow().y;
                        let b_width = b.borrow().width;
                        let b_height = b.borrow().height;
                        if c_x > b_x && c_y < b_y && c_x < b_x + b_width && c_y > b_y - b_height {
                            button_found = true;
                            match b.borrow().onclick() {
                                "pause" => {
                                    pause = true;
                                }
                                "play" => {
                                    pause = false;
                                }
                                "clear" => {
                                    sim.borrow_mut().clear();
                                }
                                "create_grid" => {
                                    let mut s = sim.borrow_mut();
                                    for i in -20..20 {
                                        for j in -20..20 {
                                            s.add_particle(i as f32, j as f32, 0.0, 0.0);
                                        }
                                    }
                                }
                                "create_sine" => {
                                    let mut s = sim.borrow_mut();
                                    let n = 100.0;
                                    for x in -n as isize..n as isize {
                                        let theta = ((x as f32)/n) * 2.0 * PI;
                                        s.add_particle(x as f32 / (n / 30.0), theta.sin() * 10.0, 0.0, 10.0_f32.sqrt() * 10.0 * theta.cos());
                                    }
                                }
                                "force_Gravity" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::Gravity);
                                    window_info.vectors_require_update = true;
                                }
                                "force_GravityResistive" => {
                                    sim.borrow_mut().set_function(
                                        simulator::SimulateFunction::GravityResistive,
                                    );
                                    window_info.vectors_require_update = true;
                                }
                                "force_Butterfly" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::Butterfly);
                                    window_info.vectors_require_update = true;
                                }
                                "force_WindowsXP" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::WindowsXP);
                                    window_info.vectors_require_update = true;
                                }
                                "force_Logistic" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::Logistic);
                                    window_info.vectors_require_update = true;
                                }
                                "force_InverseSquare" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::InverseSquare);
                                    window_info.vectors_require_update = true;
                                }
                                "force_NoForce" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::NoForce);
                                    window_info.vectors_require_update = true;
                                }
                                "force_ParallelElectric" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::ParallelElectric);
                                    window_info.vectors_require_update = true;
                                }
                                "force_Harmonic" => {
                                    sim.borrow_mut()
                                        .set_function(simulator::SimulateFunction::Harmonic);
                                    window_info.vectors_require_update = true;
                                }
                                "toggle_bounce" => {
                                    let current;
                                    {
                                        current = sim.borrow().bounce_off_walls;
                                    }
                                    sim.borrow_mut().bounce_off_walls = !current;
                                    if sim.borrow().bounce_off_walls {
                                        bounce_string = String::from("Disable Bounce");
                                    } else {
                                        bounce_string = String::from("Enable Bounce");
                                    }
                                }
                                _ => {
                                    println!("Unknown function");
                                }
                            }
                        }
                    }
                    if !button_found {
                        creating_particle = true;
                        particle_start = PhysVector {
                            x: (c_x * window_info.x_range).into(),
                            y: (c_y * window_info.y_range).into(),
                        };
                    }
                }
                sdl2::event::Event::MouseButtonUp {
                    timestamp: _ts,
                    window_id: _wi,
                    which: _wh,
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    clicks: _cl,
                    x,
                    y,
                } => {
                    if creating_particle {
                        let c_x = ((x - x_offset) as f32 * 2.0 / width as f32) - 1.0;
                        let c_y = (((y - y_offset) as f32 * 2.0 / width as f32) - 1.0) * -1.0;
                        let particle_end = PhysVector {
                            x: (c_x * window_info.x_range).into(),
                            y: (c_y * window_info.y_range).into(),
                        };
                        let vel = &particle_end + &(&particle_start * -1.0);
                        sim.borrow_mut().add_particle(
                            particle_start.x,
                            particle_start.y,
                            vel.x,
                            vel.y,
                        );
                        creating_particle = false;
                    }
                }
                _ => {}
            }
        }
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            current_buf_size =
                sim.borrow()
                    .draw_particles(&vbo, &vao, &mut current_buf_size, &window_info);
        }

        if !pause {
            Simulator::step(sim.clone(), 2);
        }

        gl_draw::draw_gridlines(&mut window_info);
        Simulator::draw_vectors(&mut window_info);

        program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.DrawArrays(gl::POINTS, 0, current_buf_size as i32);

            // gl::BindVertexArray(0);
            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        // pause_button.borrow_mut().draw();
        // play_button.borrow_mut().draw();
        // grid_button.borrow_mut().draw();
        // clear_button.borrow_mut().draw();

        for b in &buttons_vec {
            b.borrow_mut().draw();
        }

        // render text
        unsafe {
            gl.Enable(gl::BLEND);
            gl.Enable(gl::POLYGON_SMOOTH);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl.GenVertexArrays(1, &mut ft_vao);
            gl.GenBuffers(1, &mut ft_vbo);
            gl.BindVertexArray(ft_vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, ft_vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * 6 * 4) as isize,
                0 as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }

        gl_draw::render_text(
            &gl,
            "Play",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            572.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Pause",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            548.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Clear",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            524.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Grid",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            500.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Grid",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            500.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Sine",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            110.0,
            500.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Gravity",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            476.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Gravity + Resistive Force",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            452.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Butterfly",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            428.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Windows XP",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            404.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Logistic",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            380.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Inverse Square",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            356.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Parallel Electric",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            332.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "Harmonic",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            308.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            "No Force",
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            30.0,
            286.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            &bounce_string,
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            110.0,
            548.0,
            0.25,
        );

        gl_draw::render_text(
            &gl,
            &format!("Time: {:.*}", 1, sim.borrow().time()),
            &ft_face,
            &ft_program,
            &mut ft_vao,
            &mut ft_vbo,
            &mut window_info,
            100.0,
            566.0,
            0.5,
        );

        window.gl_swap_window();
    }
}
