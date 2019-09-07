extern crate glm;


pub struct Camera {
    pub speed: f32,
    pub up: glm::Vector3<f32>,

    pub position: glm::Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,

    pub x_axis: glm::Vector3<f32>,
    pub y_axis: glm::Vector3<f32>,
    pub z_axis: glm::Vector3<f32>
}

impl Camera {
    pub fn new(speed: f32, up: glm::Vector3<f32>, position: glm::Vector3<f32>) -> Camera {
        Camera {
            speed,
            up,

            position,
            pitch: 0.0,
            yaw: 0.0,

            x_axis: glm::vec3(0.0, 0.0, 0.0),
            y_axis: glm::vec3(0.0, 0.0, 0.0),
            z_axis: glm::vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn get_view_matrix(&mut self) -> glm::Matrix4<f32> {
        glm::ext::look_at(self.position, self.position - glm::vec3(0.0, 1.0, 0.0), self.up)
    }
}
