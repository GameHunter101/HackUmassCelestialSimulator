use std::intrinsics::sqrtf32;

use nalgebra::Vector3;

// Gravitational constant, can probably adjust
const GRAV: f32 = 6.67430;

struct Planet {
    // Physical properties
    mass: f32,
    pos: Vector3<f32>,
    vel: Vector3<f32>,

    // Display properties
    id: Option<PlanetType>,
    radius: f32,
    rot_vel: f32, // Angular velocity in rad/s
}

enum PlanetType {
    Star,
}

impl Planet {
    // Calculate acceleration
    fn calc_accel(&self, planet_list: &[Planet]) -> Vector3<f32> {
        // Make sure planet_list doesn't contain self
        let mut accel: Vector3<f32> = 0;
        for p in planet_list {
            let dr = p.pos - self.pos;
            accel += p.mass * dr / dr.magnitude().powi(3);
        }
        accel * GRAV
    }

    // Do the actual moving
    fn step(&mut self, planet_list: &[Planet], dt: f32,) {
        let pos_old = self.pos;
        let vel_old = self.vel;

        let accel_old = calc_accel(planet_list);
        self.pos = self.pos + self.vel * dt;                    // Expected pos
        let accel_exp = calc_accel(planet_list);
        self.vel += (accel_old + accel_exp) * 0.5 * dt;         // New vel
        self.pos = pos_old + (self.vel + vel_old) * 0.5 * dt;   // New pos
    }

    // Attempt to calculate inital centripetal velocity for stable orbit
    // For simplicity, planet_list only contains the star
    fn calc_init_velocity(&self, planet_list: &[Planet]) -> Vector3<f32> {
        let accel = calc_accel(planet_list);
        // Attempt to get a Star
        let star = Some(planet_list[0]);

        let dr = star.unwrap().pos - self.pos;
        let mag = f32::sqrt(accel.magnitude() * dr.magnitude());
        let uv = Vector3::z_axis().cross(&dr.normalize());

        uv * mag
    }
}
