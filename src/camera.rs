use nalgebra::Vector3;

#[derive(Debug, Default)]
pub struct Camera {
    pos: Vector3<f32>,
    roll: f32,
    pitch: f32,
    yaw: f32,
}

impl Camera {
    pub fn to_raw_data(&self) -> RawCameraData {
        let yaw_matrix = nalgebra::matrix![
            self.yaw.cos(), -self.yaw.sin(), 0.0;
            self.yaw.sin(), self.yaw.cos(), 0.0;
            0.0, 0.0, 1.0
        ];

        let pitch_matrix = nalgebra::matrix![
            self.pitch.cos(), 0.0, self.pitch.sin();
            0.0, 1.0, 0.0;
            -self.pitch.sin(), 0.0, self.pitch.cos()
        ];

        let roll_matrix = nalgebra::matrix![
            1.0, 0.0, 0.0;
            0.0, self.roll.cos(), -self.roll.sin();
            0.0, -self.roll.sin(), self.roll.cos()
        ];

        let rotation_matrix = yaw_matrix * pitch_matrix * roll_matrix;

        let translation = nalgebra::Translation3::from(self.pos);

        let transformation = translation.to_homogeneous() * rotation_matrix.to_homogeneous();

        RawCameraData {
            pos: self.pos.into(),
            matrix: transformation.into(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct RawCameraData {
    pos: [f32; 3],
    matrix: [[f32; 4]; 4],
}
