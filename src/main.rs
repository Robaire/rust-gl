extern crate gl;
extern crate sdl2;

use sdl2::video::GLProfile;
use sdl2::event::Event;

pub mod render;
use render::{Shader, Program};


fn main() {

    // Initialize SDL2
    let sdl_context = match sdl2::init() {
        Ok(context) => context,
        Err(message) => panic!(format!("SDL Init Failed: {}", message))
    };

    // Set GL Attributes
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attributes = video_subsystem.gl_attr();

    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_flags().debug().set();
    gl_attributes.set_context_version(4, 3);
    gl_attributes.set_multisample_buffers(1);
    gl_attributes.set_multisample_samples(4);

    // Create the window
    let window = video_subsystem
        .window("Rust OpenGL", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    assert_eq!(gl_attributes.context_profile(), GLProfile::Core);
    assert_eq!(gl_attributes.context_version(), (4, 3));

    // Create the OpenGL context
    let gl_context = window.gl_create_context().unwrap();

    // Load the OpenGL functions
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    // Load Shader
    let vertex_shader = match Shader::from_file("src/vertex.glsl", gl::VERTEX_SHADER) {
        Ok(shader) => shader,
        Err(message) => panic!(format!("Failed to create vertex shader: {}", message))
    };

    let fragment_shader =match Shader::from_file("src/fragment.glsl", gl::FRAGMENT_SHADER) {
        Ok(shader) => shader,
        Err(message) => panic!(format!("Failed to create fragment shader: {}", message))
    };

    // Create the shader program
    let program = match Program::new().attach_shader(&vertex_shader).attach_shader(&fragment_shader).link() {
        Ok(program) => program,
        Err(message) => panic!(format!("Failed to link the shader program: {}", message))
    };

    // Use Shader Program
    program.set_used();

    unsafe { gl::ClearColor(0.3, 0.3, 0.5, 1.0); }
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Enter Event Loop
    'game_loop: loop {

        // Clear the Event Queue
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'game_loop; },
                _ => {}
            };
        }

        // Draw
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        // Swap the buffers
        window.gl_swap_window();
    }
}

