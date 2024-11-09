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
        RawCameraData {
            pos: self.pos.into(),
            roll: self.roll,
            pitch: self.pitch,
            yaw: self.yaw,
        }
    }

    // Update angles of rotation from dpos[x, y] of mouse
    pub fn rotate_from_mouse(&mut self, dpos: [f64;2]) {
        self.pitch += dpos[1] as f32 * self.pitch_sens;
        self.yaw += dpos[0] as f32 * self.yaw_sens;
    }

    // Set sensitivity from argument [pitch, yaw]
    // You NEED to run this at once
    pub fn set_sensitivity(&mut self, set: [f32;2]) {
        self.pitch = set[0];
        self.yaw = set[1];
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct RawCameraData {
    pos: [f32; 3],
    roll: f32,
    pitch: f32,
    yaw: f32,
}
