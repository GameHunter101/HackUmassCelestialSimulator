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
        RawCameraData {
            pos: self.pos.into(),
            roll: self.roll,
            pitch: self.pitch,
            yaw: self.yaw,
        }
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
