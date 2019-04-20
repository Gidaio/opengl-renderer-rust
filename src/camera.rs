pub struct Camera {
    pub speed: f32,
    pub sensitivity: f32,
    pub up: glm::Vector3<f32>,

    pub position: glm::Vector3<f32>,
    pub pitch: f32,
    pub yaw: f32,

    pub x_axis: glm::Vector3<f32>,
    pub y_axis: glm::Vector3<f32>,
    pub z_axis: glm::Vector3<f32>
}

impl Camera {
    pub fn new(speed: f32, sensitivity: f32, up: glm::Vector3<f32>, position: glm::Vector3<f32>) -> Camera {
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

    pub fn get_view_matrix(&mut self) -> glm::Matrix4<f32> {
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
