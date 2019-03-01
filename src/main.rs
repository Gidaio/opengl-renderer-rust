extern crate gl;
extern crate glfw;

// use gl::types::*;
use glfw::{ Context };
use std::ffi::CString;


fn main() {
    // Initialize GLFW.
    let mut glfw_obj = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Make the window.
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw_obj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = glfw_obj.create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed).expect("Failed to create a window.");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Setup OpenGL.
    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    // Compile the shaders.
    let vertex_shader_source = CString::new(include_str!("./shader.vert")).expect("Failed to convert shader.vert to CString!");
    let vertex_shader = create_shader(vertex_shader_source, gl::VERTEX_SHADER);

    let fragment_shader_source = CString::new(include_str!("./shader.frag")).expect("Failed to convert shader.frag to CString!");
    let fragment_shader = create_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

    // Create a shader program.
    unsafe {
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for errors.
        let mut success = 1;
        gl::GetProgramiv(shader_program, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut error_length = 0;
            gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut error_length);
            let error_string = CString::from_vec_unchecked(
                std::iter::repeat(b' ').take(error_length as usize).collect::<Vec<u8>>()
            );
            gl::GetProgramInfoLog(
                shader_program,
                error_length,
                std::ptr::null_mut(),
                error_string.as_ptr() as *mut _
            );

            let error = error_string.to_string_lossy().into_owned();
            panic!("Error linking shader program: {}", error);
        }

        gl::UseProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // Make a VAO!
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    // Make a VBO for our vertices.
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0
    ];

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    // Make a vertex attribute pointer.
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<f32>() as i32 * 3,
            0 as *const _
        );
        gl::EnableVertexAttribArray(0);
    }

    // Make an EBO for a rectangle.
    let indices: [u32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&indices) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    // Main loop!
    while !window.should_close() {
        // Do rendering stuff.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
        window.swap_buffers();

        // Handle events.
        glfw_obj.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(horiz, vert) => {
                    unsafe { gl::Viewport(0, 0, horiz, vert); }
                }

                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, mods) => {
                    if mods.contains(glfw::Modifiers::Shift) {
                        println!("You were pressing shift!");
                    }
                    window.set_should_close(true);
                }

                _ => {}
            }
        }
    }
}


fn create_shader(source: CString, shader_type: u32) -> u32 {
    unsafe {
        let shader_id = gl::CreateShader(shader_type);
        gl::ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);

        // Check for errors.
        let mut success = 1;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut error_length = 0;
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_length);
            let error_string = CString::from_vec_unchecked(
                std::iter::repeat(b' ').take(error_length as usize).collect::<Vec<u8>>()
            );
            gl::GetShaderInfoLog(
                shader_id,
                error_length,
                std::ptr::null_mut(),
                error_string.as_ptr() as *mut _
            );

            let error = error_string.to_string_lossy().into_owned();
            panic!("Error compiling shader: {}", error);
        }

        return shader_id;
    }
}
