extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;

use gl::types::*;
use glfw::{ Context };
use image::GenericImageView;
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
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
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
    let vertices: [f32; 32] = [
        0.5, 0.5, 0.0,    1.0, 0.0, 0.0,  1.0, 0.0,
        -0.5, 0.5, 0.0,   1.0, 1.0, 0.0,  0.0, 0.0,
        -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 1.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,  1.0, 1.0
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

    let triangles: [i32; 6] = [
        0, 1, 2,
        2, 3, 0
    ];

    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            std::mem::size_of_val(&triangles) as isize,
            triangles.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    // Make the vertex attribute pointers.
    create_vertex_attribute_array::<f32>(0, 3, 8, 0);
    create_vertex_attribute_array::<f32>(1, 3, 8, 3);
    create_vertex_attribute_array::<f32>(2, 2, 8, 6);

    // Set the textures!
    let _wall_texture = create_texture("./src/wall.jpg", gl::TEXTURE0, gl::RGB);
    let _face_texture = create_texture("./src/awesomeface.png", gl::TEXTURE1, gl::RGBA);
    let uniform_0_name = CString::new("texture1").unwrap();
    let uniform_1_name = CString::new("texture2").unwrap();
    unsafe {
        let uniform_0_location = gl::GetUniformLocation(shader_program, uniform_0_name.as_ptr());
        let uniform_1_location = gl::GetUniformLocation(shader_program, uniform_1_name.as_ptr());
        gl::Uniform1i(uniform_0_location, 0);
        gl::Uniform1i(uniform_1_location, 1);
    }

    // Build the transformation matrix.
    let base_matrix = glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let rotation = glm::ext::rotate(&base_matrix, glm::radians(90.0), glm::vec3(0.0, 0.0, 1.0));
    let scale = glm::ext::scale(&rotation, glm::vec3(0.5, 0.5, 0.5));

    // Set the transform matrix.
    let transform_matrix_name = CString::new("transform").unwrap();
    unsafe {
        let transform_matrix_location = gl::GetUniformLocation(shader_program, transform_matrix_name.as_ptr());
        gl::UniformMatrix4fv(transform_matrix_location, 1, gl::FALSE, scale.as_array()[0].as_array().as_ptr());
    }

    // Get our blend uniform's location.
    let blend_uniform_name = CString::new("blend").unwrap();
    let blend_uniform_location = unsafe { gl::GetUniformLocation(shader_program, blend_uniform_name.as_ptr()) };
    println!("Blend uniform location {}", blend_uniform_location);

    let mut blend = 0.2;
    let mut previous_time = glfw_obj.get_time();

    // Main loop!
    while !window.should_close() {
        // Do rendering stuff.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::Uniform1f(blend_uniform_location, blend);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
        window.swap_buffers();

        // Get our timer going.
        let current_time = glfw_obj.get_time();
        let elapsed_time = current_time - previous_time;
        previous_time = current_time;

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

                glfw::WindowEvent::Key(glfw::Key::Up, _, glfw::Action::Repeat, _) => {
                    println!("Pressing up!");
                    blend += (elapsed_time * 100.0) as f32;
                    if blend > 1.0 {
                        blend = 1.0;
                    }
                }

                glfw::WindowEvent::Key(glfw::Key::Down, _, glfw::Action::Repeat, _) => {
                    println!("Pressing down!");
                    blend -= (elapsed_time * 100.0) as f32;
                    if blend < 0.0 {
                        blend = 0.0;
                    }
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


trait HasOpenGLType {
    fn get_opengl_type() -> GLenum;
}

impl HasOpenGLType for f32 {
    fn get_opengl_type() -> GLenum {
        gl::FLOAT
    }
}

fn create_vertex_attribute_array<T: HasOpenGLType>(index: u32, size: i32, stride: i32, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
            T::get_opengl_type(),
            gl::FALSE,
            std::mem::size_of::<T>() as i32 * stride,
            (std::mem::size_of::<f32>() * offset) as *const _
        );
        gl::EnableVertexAttribArray(index);
    }
}

fn create_texture(path: &'static str, texture_spot: u32, pixel_type: u32) -> u32 {
    // Load up the image.
    let image_obj = image::open(path).unwrap();
    let (image_width, image_height) = image_obj.dimensions();
    let image_data = image_obj.raw_pixels();

    // Load the texture.
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(texture_spot);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            image_width as i32,
            image_height as i32,
            0,
            pixel_type,
            gl::UNSIGNED_BYTE,
            image_data.as_ptr() as *const _
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    return texture;
}
