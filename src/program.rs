use std::ffi::CString;
use std::fs;


pub struct Program {
  id: u32
}

impl Program {
  pub fn new(shader_name: &str) -> Program {
    let vertex_shader_source = fs::read_to_string(format!("./assets/{}.vert", shader_name))
        .expect(&format!("Failed to load {} vertex shader!", shader_name));
    let vertex_shader_source = CString::new(vertex_shader_source).unwrap();
    let vertex_shader = Program::create_shader(vertex_shader_source, gl::VERTEX_SHADER);

    let fragment_shader_source = fs::read_to_string(format!("./assets/{}.frag", shader_name))
        .expect(&format!("Failed to load {} fragment shader!", shader_name));
    let fragment_shader_source = CString::new(fragment_shader_source).unwrap();
    let fragment_shader = Program::create_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

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

    Program { id: shader_program }
  }

  pub fn set_used(&self) {
    unsafe {
      gl::UseProgram(self.id);
    }
  }

  pub fn set_float(&self, uniform_name: &'static str, float: f32) {
    let uniform_location = self.get_uniform_location(uniform_name);
    unsafe {
      gl::Uniform1f(uniform_location, float);
    }
  }

  pub fn set_vector3(&self, uniform_name: &'static str, vector: glm::Vector3<f32>) {
    let uniform_location = self.get_uniform_location(uniform_name);
    unsafe {
      gl::Uniform3fv(uniform_location, 1, vector.as_array().as_ptr());
    }
  }

  pub fn set_matrix(&self, uniform_name: &'static str, matrix: glm::Matrix4<f32>) {
    let uniform_location = self.get_uniform_location(uniform_name);
    unsafe {
      gl::UniformMatrix4fv(uniform_location, 1, gl::FALSE, matrix.as_array()[0].as_array().as_ptr());
    }
  }

  fn get_uniform_location(&self, uniform_name: &'static str) -> i32 {
    let uniform_cstring = CString::new(uniform_name).unwrap();
    unsafe { gl::GetUniformLocation(self.id, uniform_cstring.as_ptr()) }
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
}
