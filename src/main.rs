extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;

use gl::types::*;
use glfw::{ Context };
use image::GenericImageView;
use std::ffi::CString;


struct Camera {
    speed: f32,
    sensitivity: f32,
    up: glm::Vector3<f32>,

    position: glm::Vector3<f32>,
    pitch: f32,
    yaw: f32,

    x_axis: glm::Vector3<f32>,
    y_axis: glm::Vector3<f32>,
    z_axis: glm::Vector3<f32>
}

impl Camera {
    fn new(speed: f32, sensitivity: f32, up: glm::Vector3<f32>, position: glm::Vector3<f32>) -> Camera {
        Camera {
            speed,
            sensitivity,
            up,

            position,
            pitch: 0.0,
            yaw: 0.0,

            x_axis: glm::vec3(0.0, 0.0, 0.0),
            y_axis: glm::vec3(0.0, 0.0, 0.0),
            z_axis: glm::vec3(0.0, 0.0, 0.0)
        }
    }

    fn get_view_matrix(&mut self) -> glm::Matrix4<f32> {
        self.z_axis = glm::builtin::normalize(glm::vec3(
            -((glm::radians(self.pitch) as f32).cos() * (glm::radians(self.yaw) as f32).cos()),
            -(glm::radians(self.pitch) as f32).sin(),
            -((glm::radians(self.pitch) as f32).cos() * (glm::radians(self.yaw) as f32).sin())
        ));

        self.x_axis = glm::builtin::normalize(
            glm::builtin::cross(self.up, self.z_axis)
        );
        self.y_axis = glm::builtin::cross(self.z_axis, self.x_axis);

        let rotation_matrix = glm::mat4(
            self.x_axis.x, self.y_axis.x, self.z_axis.x, 0.0,
            self.x_axis.y, self.y_axis.y, self.z_axis.y, 0.0,
            self.x_axis.z, self.y_axis.z, self.z_axis.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let position_matrix = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -self.position.x, -self.position.y, -self.position.z, 1.0
        );

        rotation_matrix * position_matrix
    }
}


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
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Enable(gl::DEPTH_TEST);
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
    let vertices: [f32; 40] = [
        0.5, 0.5, 0.5,     1.0, 0.0,
        -0.5, 0.5, 0.5,    0.0, 0.0,
        -0.5, -0.5, 0.5,   0.0, 1.0,
        0.5, -0.5, 0.5,    1.0, 1.0,
        0.5, 0.5, -0.5,    0.0, 0.0,
        -0.5, 0.5, -0.5,   1.0, 0.0,
        -0.5, -0.5, -0.5,  1.0, 1.0,
        0.5, -0.5, -0.5,   0.0, 1.0
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

    let triangles: [i32; 36] = [
        0, 1, 2,  2, 3, 0,
        4, 5, 1,  1, 0, 4,
        1, 5, 6,  6, 2, 1,
        5, 4, 7,  7, 6, 5,
        4, 0, 3,  3, 7, 4,
        3, 2, 6,  6, 7, 3,
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
    create_vertex_attribute_array::<f32>(0, 3, 5, 0);
    create_vertex_attribute_array::<f32>(1, 2, 5, 3);

    // Set the textures!
    let _wall_texture = create_texture("./src/wall.jpg", gl::TEXTURE0, gl::RGB);
    let _face_texture = create_texture("./src/awesomeface.png", gl::TEXTURE1, gl::RGBA);

    let texture1_location = get_uniform_location(shader_program, "texture1");
    let texture2_location = get_uniform_location(shader_program, "texture2");
    unsafe {
        gl::Uniform1i(texture1_location, 0);
        gl::Uniform1i(texture2_location, 1);
    }

    // Get our matrix locations.
    let model_matrix_location = get_uniform_location(shader_program, "model");
    let view_matrix_location = get_uniform_location(shader_program, "view");
    let projection_matrix_location = get_uniform_location(shader_program, "projection");

    // Set up the projection matrix (this doesn't change).
    let projection_matrix = glm::ext::perspective(glm::radians(45.0), window_width as f32 / window_height as f32, 0.1, 100.0);

    // Lots of cubes!
    let cube_positions: [glm::Vector3<f32>; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5)
    ];

    // Set up the basic camera.
    let mut camera = Camera::new(5.0, 0.5, glm::vec3(0.0, 1.0, 0.0), glm::vec3(0.0, 0.0, 5.0));

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

        // Do rendering stuff.
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UniformMatrix4fv(view_matrix_location, 1, gl::FALSE, view_matrix.as_array()[0].as_array().as_ptr());
            gl::UniformMatrix4fv(projection_matrix_location, 1, gl::FALSE, projection_matrix.as_array()[0].as_array().as_ptr());

            for cube_index in 0..cube_positions.len() {
                let mut model_matrix = glm::ext::translate(&identity_matrix(), cube_positions[cube_index]);
                let angle = current_time * 50.0 + 20.0 * cube_index as f32;
                model_matrix = glm::ext::rotate(&model_matrix, glm::radians(angle), glm::vec3(0.5, 1.0, 0.0));

                gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, model_matrix.as_array()[0].as_array().as_ptr());
                gl::DrawElements(gl::TRIANGLES, 36, gl::UNSIGNED_INT, 0 as *const _);
            }
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
