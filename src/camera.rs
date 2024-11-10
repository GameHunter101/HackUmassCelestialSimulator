use nalgebra::{Matrix3, Vector3};

#[derive(Debug, Default)]
pub struct Camera {
    pub pos: Vector3<f32>,
    // Angles of rotation
    pub roll: f32, // Unchanged
    pub pitch: f32,
    pub yaw: f32,

    // Sensitivity settings
    pitch_sens: f32,
    yaw_sens: f32,
}

impl Camera {
    fn get_rotation_matrix(&self) -> Matrix3<f32> {
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

        yaw_matrix * pitch_matrix * roll_matrix
    }

    pub fn to_raw_data(&self) -> RawCameraData {
        let rotation_matrix = self.get_rotation_matrix();

        // println!("Pos: {}", self.pos);

        RawCameraData {
            pos: (rotation_matrix * self.pos).to_homogeneous().into(),
            matrix: rotation_matrix.to_homogeneous().into(),
            /* pos: self.pos.into(),
            padding: [0.0; 4], */
        }
    }

    // Update angles of rotation from dpos[x, y] of mouse
    pub fn rotate_from_mouse(&mut self, dpos: [f64; 2]) {
        self.pitch -= (dpos[0] as f32) * self.pitch_sens;
        // self.pitch = std::f32::consts::PI;
        // self.yaw += (dpos[0] as f32) * self.yaw_sens;
        // println!("rot: {}", self.get_rotation_matrix());
    }

    // Set sensitivity from argument [pitch, yaw]
    // You NEED to run this at once
    pub fn set_sensitivity(&mut self, set: [f32; 2]) {
        self.pitch_sens = set[0] / 100.0;
        self.yaw_sens = set[1] / 100.0;
    }

    pub fn scroll(&mut self, delta: f32) {
        self.pos *= 1.0 + delta;
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct RawCameraData {
    pos: [f32; 4],
    matrix: [[f32; 4]; 4],
    /* a: [f32;3],
    b: [f32;3],
    c: [f32;3], */
    // padding: [f32;3]
    /* pos: [f32; 3],
    padding: [f32; 4], */
}
