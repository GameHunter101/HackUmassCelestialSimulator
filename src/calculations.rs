use nalgebra::Vector2;

const GRAV: f32 = 6.67430;

struct Planet {
    mass: f32,
    radius: f32, // Display size, no calculation
    pos: Vector3<f32>,
    vel: Vector3<f32>,
    accel: Vector3<f32>, // Do we need this?
}

