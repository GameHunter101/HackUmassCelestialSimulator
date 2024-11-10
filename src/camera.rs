use nalgebra::{Vector2, Vector3};

#[derive(Debug, Default)]
pub struct Camera {
    pos: Vector3<f32>,
    // Angles of rotation
    roll: f32,  // Unchanged
    pitch: f32,
    yaw: f32,

    // Sensitivity settings
    pitch_sens: f32,
    yaw_sens: f32,
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

        let rotation_matrix = (yaw_matrix * pitch_matrix * roll_matrix).to_homogeneous();

        println!("mat: {rotation_matrix}");

        RawCameraData {
            pos: self.pos.to_homogeneous().into(),
            matrix: rotation_matrix.into(),
            /* pos: self.pos.into(),
            padding: [0.0; 4], */
        }
    }

    // Update angles of rotation from dpos[x, y] of mouse
    pub fn rotate_from_mouse(&mut self, dpos: [f64;2]) {
        self.pitch += (dpos[1] as f32) * self.pitch_sens;
        self.yaw += (dpos[0] as f32) * self.yaw_sens;
    }

    // Set sensitivity from argument [pitch, yaw]
    // You NEED to run this at once
    pub fn set_sensitivity(&mut self, set: [f32;2]) {
        self.pitch_sens = set[0] / 100.0;
        self.yaw_sens = set[1] / 100.0;
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct RawCameraData {
    pos: [f32;4],
    matrix: [[f32; 4]; 4],
    /* a: [f32;3],
    b: [f32;3],
    c: [f32;3], */
    // padding: [f32;3]
    /* pos: [f32; 3],
    padding: [f32; 4], */
}
