extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;

use gl::types::*;
use glfw::{ Context };
use image::GenericImageView;
use std::ffi::CString;
use std::fs;

mod camera;


fn main() {
    // Initialize GLFW.
    let mut glfw_obj = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Make the window.
    let window_width = 800;
    let window_height = 600;
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw_obj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = glfw_obj.create_window(window_width, window_height, "LearnOpenGL", glfw::WindowMode::Windowed).expect("Failed to create a window.");
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Setup OpenGL.
    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    // Create a shader program.
    let target_shader_program = create_program("target");

    // Make a VAO!
    let mut target_vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut target_vao);
        gl::BindVertexArray(target_vao);
    }

    // Make a VBO for our cube mesh.
    let cube_mesh: [f32; 216] = [
        // Front face
         0.5,  0.5,  0.5,   0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,

        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,   0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,   0.0,  0.0,  1.0,

        // Top face
         0.5,  0.5, -0.5,   0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,

        -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,   0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,   0.0,  1.0,  0.0,

        // Left face
        -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,

        // Back face
        -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,   0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,   0.0,  0.0, -1.0,

         0.5, -0.5, -0.5,   0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,

        // Right face
         0.5,  0.5, -0.5,   1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,   1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,   1.0,  0.0,  0.0,

         0.5, -0.5,  0.5,   1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,   1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,   1.0,  0.0,  0.0,

        // Bottom face
         0.5, -0.5,  0.5,   0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,

        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,   0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,   0.0, -1.0,  0.0
    ];

    let mut cube_vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut cube_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&cube_mesh) as isize,
            cube_mesh.as_ptr() as *const _,
            gl::STATIC_DRAW
        );
    }

    // Make the vertex attribute pointers.
    create_vertex_attribute_array::<f32>(0, 3, 6, 0);
    create_vertex_attribute_array::<f32>(1, 3, 6, 3);

    // Get our matrix locations.
    let target_model_matrix_location = get_uniform_location(target_shader_program, "model");
    let target_view_matrix_location = get_uniform_location(target_shader_program, "view");
    let target_projection_matrix_location = get_uniform_location(target_shader_program, "projection");

    let target_object_color_location = get_uniform_location(target_shader_program, "objectColor");
    let target_light_color_location = get_uniform_location(target_shader_program, "lightColor");
    let target_light_position_location = get_uniform_location(target_shader_program, "lightPosition");
    let target_viewer_position_location = get_uniform_location(target_shader_program, "viewerPosition");

    // Make a new shader for our lamp.
    let lamp_shader_program = create_program("lamp");

    // Make a new VAO for the lamp.
    let mut lamp_vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut lamp_vao);
        gl::BindVertexArray(lamp_vao);
    }

    // Reuse the same mesh (VBO) for the lamp.
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
    }

    // Make a new attribute array for it. We leave out the normals, because they're not important.
    create_vertex_attribute_array::<f32>(0, 3, 6, 0);

    // Get our matrix locations.
    let lamp_model_matrix_location = get_uniform_location(lamp_shader_program, "model");
    let lamp_view_matrix_location = get_uniform_location(lamp_shader_program, "view");
    let lamp_projection_matrix_location = get_uniform_location(lamp_shader_program, "projection");

    // Set up the projection matrix (this doesn't change).
    let projection_matrix = glm::ext::perspective(glm::radians(45.0), window_width as f32 / window_height as f32, 0.1, 100.0);

    // Set up the basic camera.
    let mut camera = camera::Camera::new(5.0, 0.1, glm::vec3(0.0, 1.0, 0.0), glm::vec3(0.0, 0.0, 5.0));

    // This is for mouse input.
    let mut first_mouse_input = true;
    let mut previous_cursor_x = -1.0;
    let mut previous_cursor_y = -1.0;
    let mut previous_time = glfw_obj.get_time() as f32;

    // Main loop!
    while !window.should_close() {
        // Get our timer going.
        let current_time = glfw_obj.get_time() as f32;
        let delta_time = current_time - previous_time;
        previous_time = current_time;

        let view_matrix = camera.get_view_matrix();

        let light_x = current_time.cos() * 1.2;
        let light_y = 1.0;
        let light_z = current_time.sin() * 2.0;
        let light_position = glm::vec3(light_x, light_y, light_z);

        // Do rendering stuff.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Render the target cube.
            gl::UseProgram(target_shader_program);
            gl::UniformMatrix4fv(target_model_matrix_location, 1, gl::FALSE, identity_matrix().as_array()[0].as_array().as_ptr());
            gl::UniformMatrix4fv(target_view_matrix_location, 1, gl::FALSE, view_matrix.as_array()[0].as_array().as_ptr());
            gl::UniformMatrix4fv(target_projection_matrix_location, 1, gl::FALSE, projection_matrix.as_array()[0].as_array().as_ptr());
            gl::Uniform3f(target_object_color_location, 1.0, 0.5, 0.31);
            gl::Uniform3f(target_light_color_location, 1.0, 1.0, 1.0);
            gl::Uniform3fv(target_light_position_location, 1, light_position.as_array().as_ptr());
            gl::Uniform3fv(target_viewer_position_location, 1, camera.position.as_array().as_ptr());

            gl::BindVertexArray(target_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            // Render the lamp cube.
            gl::UseProgram(lamp_shader_program);
            let model_matrix = glm::ext::translate(&identity_matrix(), light_position);
            let model_matrix = glm::ext::scale(&model_matrix, glm::vec3(0.2, 0.2, 0.2));

            gl::UniformMatrix4fv(lamp_model_matrix_location, 1, gl::FALSE, model_matrix.as_array()[0].as_array().as_ptr());
            gl::UniformMatrix4fv(lamp_view_matrix_location, 1, gl::FALSE, view_matrix.as_array()[0].as_array().as_ptr());
            gl::UniformMatrix4fv(lamp_projection_matrix_location, 1, gl::FALSE, projection_matrix.as_array()[0].as_array().as_ptr());

            gl::BindVertexArray(lamp_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
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

                glfw::WindowEvent::CursorPos(cursor_x, cursor_y) => {
                    if first_mouse_input {
                        previous_cursor_x = cursor_x;
                        previous_cursor_y = cursor_y;
                        first_mouse_input = false;
                    }

                    let cursor_delta_x = cursor_x - previous_cursor_x;
                    let cursor_delta_y = cursor_y - previous_cursor_y;

                    previous_cursor_x = cursor_x;
                    previous_cursor_y = cursor_y;

                    camera.pitch = camera.pitch - cursor_delta_y as f32 * camera.sensitivity;
                    camera.yaw = camera.yaw + cursor_delta_x as f32 * camera.sensitivity;

                    if camera.pitch > 89.0 {
                        camera.pitch = 89.0
                    }
                    else if camera.pitch < -89.0 {
                        camera.pitch = -89.0
                    }
                }

                _ => {}
            }
        }

        if window.get_key(glfw::Key::E) == glfw::Action::Press {
            camera.position = camera.position - camera.z_axis * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            camera.position = camera.position + camera.z_axis * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::F) == glfw::Action::Press {
            camera.position = camera.position + camera.x_axis * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            camera.position = camera.position - camera.x_axis * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            camera.position = camera.position + camera.up * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            camera.position = camera.position - camera.up * camera.speed * delta_time;
        }
    }
}


fn create_program(shader_name: &str) -> u32 {
    let vertex_shader_source = fs::read_to_string(format!("./assets/{}.vert", shader_name))
        .expect(&format!("Failed to load {} vertex shader!", shader_name));
    let vertex_shader_source = CString::new(vertex_shader_source).unwrap();
    let vertex_shader = create_shader(vertex_shader_source, gl::VERTEX_SHADER);

    let fragment_shader_source = fs::read_to_string(format!("./assets/{}.frag", shader_name))
        .expect(&format!("Failed to load {} fragment shader!", shader_name));
    let fragment_shader_source = CString::new(fragment_shader_source).unwrap();
    let fragment_shader = create_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

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

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
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

#[allow(dead_code)]
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

fn get_uniform_location(shader_program: u32, uniform_name: &'static str) -> i32 {
    let uniform_cstring = CString::new(uniform_name).unwrap();
    unsafe { gl::GetUniformLocation(shader_program, uniform_cstring.as_ptr()) }
}

fn identity_matrix() -> glm::Matrix4<f32> {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
