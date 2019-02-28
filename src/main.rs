extern crate gl;
extern crate glfw;

use glfw::{ Context };


fn main() {
    let mut glfw_obj = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw_obj.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw_obj.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw_obj.create_window(800, 600, "LearnOpenGL", glfw::WindowMode::Windowed).expect("Failed to create a window.");

    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        window.swap_buffers();
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
