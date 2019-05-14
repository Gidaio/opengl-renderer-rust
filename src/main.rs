extern crate gl;
extern crate glfw;
extern crate glm;
extern crate image;

use gl::types::*;
use glfw::{ Context };
use image::GenericImageView;

mod camera;
mod program;


fn main() {
    // Initialize GLFW.
    let mut glfw_obj = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Make the window.
    let window_width = 800;
    let window_height = 600;
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw_obj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = glfw_obj.create_window(window_width, window_height, "Learn OpenGL", glfw::WindowMode::Windowed).expect("Failed to create a window.");
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
    let target_shader_program = program::Program::new("target");

    // Make a VAO!
    let mut target_vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut target_vao);
        gl::BindVertexArray(target_vao);
    }

    // Make a VBO for our cube mesh.
    let cube_mesh: [f32; 288] = [
        // Front face
         0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  0.0, 0.0,

        -0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  0.0, 0.0,
         0.5, -0.5,  0.5,   0.0,  0.0,  1.0,  1.0, 0.0,
         0.5,  0.5,  0.5,   0.0,  0.0,  1.0,  1.0, 1.0,

        // Top face
         0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  1.0, 0.0,
        -0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  0.0, 0.0,
        -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  0.0, 1.0,

        -0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  0.0, 1.0,
         0.5,  0.5,  0.5,   0.0,  1.0,  0.0,  1.0, 1.0,
         0.5,  0.5, -0.5,   0.0,  1.0,  0.0,  1.0, 0.0,

        // Left face
        -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,  1.0, 1.0,
        -0.5,  0.5, -0.5,  -1.0,  0.0,  0.0,  1.0, 0.0,
        -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,  0.0, 0.0,

        -0.5, -0.5, -0.5,  -1.0,  0.0,  0.0,  0.0, 0.0,
        -0.5, -0.5,  0.5,  -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5,  0.5,  0.5,  -1.0,  0.0,  0.0,  1.0, 1.0,

        // Back face
        -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  0.0, 1.0,
         0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  1.0, 1.0,
         0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  1.0, 0.0,

         0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  1.0, 0.0,
        -0.5, -0.5, -0.5,   0.0,  0.0, -1.0,  0.0, 0.0,
        -0.5,  0.5, -0.5,   0.0,  0.0, -1.0,  0.0, 1.0,

        // Right face
         0.5,  0.5, -0.5,   1.0,  0.0,  0.0,  1.0, 0.0,
         0.5,  0.5,  0.5,   1.0,  0.0,  0.0,  1.0, 1.0,
         0.5, -0.5,  0.5,   1.0,  0.0,  0.0,  0.0, 1.0,

         0.5, -0.5,  0.5,   1.0,  0.0,  0.0,  0.0, 1.0,
         0.5, -0.5, -0.5,   1.0,  0.0,  0.0,  0.0, 0.0,
         0.5,  0.5, -0.5,   1.0,  0.0,  0.0,  1.0, 0.0,

        // Bottom face
         0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  1.0, 1.0,
        -0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  0.0, 1.0,
        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  0.0, 0.0,

        -0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  0.0, 0.0,
         0.5, -0.5, -0.5,   0.0, -1.0,  0.0,  1.0, 0.0,
         0.5, -0.5,  0.5,   0.0, -1.0,  0.0,  1.0, 1.0
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
    create_vertex_attribute_array::<f32>(0, 3, 8, 0);
    create_vertex_attribute_array::<f32>(1, 3, 8, 3);
    create_vertex_attribute_array::<f32>(2, 2, 8, 6);

    // Get the texture.
    let _container_diffuse = create_texture("./assets/container.diffuse.png", gl::TEXTURE0, gl::RGBA);
    let _container_specular = create_texture("./assets/container.specular.png", gl::TEXTURE1, gl::RGBA);

    // Set the unchanging uniforms.
    let point_lights: [glm::Vector3<f32>; 4] = [
        glm::vec3( 0.7,  0.2,  2.0),
        glm::vec3( 2.3, -3.3, -4.0),
        glm::vec3(-4.0,  2.0, -12.0),
        glm::vec3( 0.0,  0.0, -3.0)
    ];

    target_shader_program.set_used();
    target_shader_program.set_signed_int("material.diffuseMap", 0);
    target_shader_program.set_signed_int("material.specularMap", 1);
    target_shader_program.set_float("material.shininess", 32.0);

    target_shader_program.set_vector3("directionalLight.direction", glm::vec3(-0.2, -1.0, -0.3));
    target_shader_program.set_vector3("light.colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("light.colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("light.colors.specular", glm::vec3(1.0, 1.0, 1.0));

    target_shader_program.set_vector3("pointLights[0].position", point_lights[0]);
    target_shader_program.set_vector3("pointLights[0].colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("pointLights[0].colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("pointLights[0].colors.specular", glm::vec3(1.0, 1.0, 1.0));
    target_shader_program.set_float("pointLights[0].attenuation.constant", 1.0);
    target_shader_program.set_float("pointLights[0].attenuation.linear", 0.09);
    target_shader_program.set_float("pointLights[0].attenuation.quadratic", 0.032);

    target_shader_program.set_vector3("pointLights[1].position", point_lights[1]);
    target_shader_program.set_vector3("pointLights[1].colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("pointLights[1].colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("pointLights[1].colors.specular", glm::vec3(1.0, 1.0, 1.0));
    target_shader_program.set_float("pointLights[1].attenuation.constant", 1.0);
    target_shader_program.set_float("pointLights[1].attenuation.linear", 0.09);
    target_shader_program.set_float("pointLights[1].attenuation.quadratic", 0.032);

    target_shader_program.set_vector3("pointLights[2].position", point_lights[2]);
    target_shader_program.set_vector3("pointLights[2].colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("pointLights[2].colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("pointLights[2].colors.specular", glm::vec3(1.0, 1.0, 1.0));
    target_shader_program.set_float("pointLights[2].attenuation.constant", 1.0);
    target_shader_program.set_float("pointLights[2].attenuation.linear", 0.09);
    target_shader_program.set_float("pointLights[2].attenuation.quadratic", 0.032);

    target_shader_program.set_vector3("pointLights[3].position", point_lights[3]);
    target_shader_program.set_vector3("pointLights[3].colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("pointLights[3].colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("pointLights[3].colors.specular", glm::vec3(1.0, 1.0, 1.0));
    target_shader_program.set_float("pointLights[3].attenuation.constant", 1.0);
    target_shader_program.set_float("pointLights[3].attenuation.linear", 0.09);
    target_shader_program.set_float("pointLights[3].attenuation.quadratic", 0.032);

    target_shader_program.set_float("spotlight.innerCutoff", (glm::radians(12.5) as f32).cos());
    target_shader_program.set_float("spotlight.outerCutoff", (glm::radians(17.0) as f32).cos());
    target_shader_program.set_vector3("spotlight.colors.ambient", glm::vec3(0.2, 0.2, 0.2));
    target_shader_program.set_vector3("spotlight.colors.diffuse", glm::vec3(0.5, 0.5, 0.5));
    target_shader_program.set_vector3("spotlight.colors.specular", glm::vec3(1.0, 1.0, 1.0));
    target_shader_program.set_float("spotlight.attenuation.constant", 1.0);
    target_shader_program.set_float("spotlight.attenuation.linear", 0.09);
    target_shader_program.set_float("spotlight.attenuation.quadratic", 0.032);

    // Make a new shader for our lamp.
    let lamp_shader_program = program::Program::new("lamp");

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

    // Make a new attribute array for it. We leave out the normals and texture coordinates,
    // because they're not important.
    create_vertex_attribute_array::<f32>(0, 3, 8, 0);

    // Set up the projection matrix (this doesn't change).
    let projection_matrix = glm::ext::perspective(glm::radians(45.0), window_width as f32 / window_height as f32, 0.1, 100.0);

    // Set up the basic camera.
    let mut camera = camera::Camera::new(5.0, 0.1, glm::vec3(0.0, 1.0, 0.0), glm::vec3(0.0, 0.0, 5.0));

    // This is for mouse input.
    let mut first_mouse_input = true;
    let mut previous_cursor_x = -1.0;
    let mut previous_cursor_y = -1.0;
    let mut previous_time = glfw_obj.get_time() as f32;

    let cube_positions: [glm::Vector3<f32>; 10] = [
        glm::vec3( 0.0,  0.0,  0.0),
        glm::vec3( 2.0,  5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3( 2.4, -0.4, -3.5),
        glm::vec3(-1.7,  3.0, -7.5),
        glm::vec3( 1.3, -2.0, -2.5),
        glm::vec3( 1.5,  2.0, -2.5),
        glm::vec3( 1.5,  0.2, -1.5),
        glm::vec3(-1.3,  1.0, -1.5)
    ];

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

            // Render the target cube.
            target_shader_program.set_used();

            target_shader_program.set_vector3("viewerPosition", camera.position);
            target_shader_program.set_vector3("spotlight.position", camera.position);
            target_shader_program.set_vector3("spotlight.direction", -camera.z_axis);

            target_shader_program.set_matrix("view", view_matrix);
            target_shader_program.set_matrix("projection", projection_matrix);

            gl::BindVertexArray(target_vao);

            for i in 0..10 {
                let mut model_matrix = glm::ext::translate(&identity_matrix(), cube_positions[i]);
                let angle = 20.0 * i as f32;
                model_matrix = glm::ext::rotate(&model_matrix, glm::radians(angle), glm::vec3(1.0, 0.3, 0.5));
                target_shader_program.set_matrix("model", model_matrix);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            // Render the lamp cube.
            lamp_shader_program.set_used();
            gl::BindVertexArray(lamp_vao);

            for light_position in point_lights.iter() {
                let mut model_matrix = glm::ext::translate(&identity_matrix(), *light_position);
                model_matrix = glm::ext::scale(&model_matrix, glm::vec3(0.2, 0.2, 0.2));

                lamp_shader_program.set_matrix("model", model_matrix);
                lamp_shader_program.set_matrix("view", view_matrix);
                lamp_shader_program.set_matrix("projection", projection_matrix);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            camera.position = camera.position + camera.up * camera.speed * delta_time;
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            camera.position = camera.position - camera.up * camera.speed * delta_time;
        }
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

fn identity_matrix() -> glm::Matrix4<f32> {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
