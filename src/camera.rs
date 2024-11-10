use nalgebra::{Matrix4, Vector3};

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
    fn get_rotation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_euler_angles(self.roll, self.pitch, self.yaw)
    }

    pub fn to_raw_data(&self) -> RawCameraData {
        let rotation_matrix = self.get_rotation_matrix();

        // println!("Pos: {}", self.pos);

        RawCameraData {
            pos: (rotation_matrix * self.pos.to_homogeneous()).into(),
            matrix: rotation_matrix.into(),
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
        let delta = 1.0 + delta;
        if delta > 1.0 && self.pos.magnitude_squared() < 4000000.0 {
            self.pos *= delta;
        }

        if delta < 1.0 && self.pos.magnitude_squared() > 0.01 {
            self.pos *= delta;
        }
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
