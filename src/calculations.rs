use nalgebra::Vector3;

const GRAV: f32 = 6.67430;

struct Planet {
    mass: f32,
    radius: f32, // Display size, no calculation
    pos: Vector3<f32>,
    vel: Vector3<f32>,
}

impl Planet {
    // Calculate acceleration
    fn calcAccel(&mut self, planetList: &[Planet]) -> Vector3<f32> {
        // Make sure planetList doesn't contain self
        let mut accel: Vector3<f32> = 0;
        for p in planetList {
            let dr = p.pos - self.pos;
            accel += p.mass * dr / dr.magnitude().powi(3);
        }
        accel * GRAV
    }

    // Do the actual moving
    fn step(&mut self, planetList: &[Planet], dt: f32,) {
        let pos_old = self.pos;
        let vel_old = self.vel;
        let accel_old = calcAccel(planetList);
        self.pos = self.pos + self.vel * dt;                    // Expected pos
        let accel_exp = calcAccel(planetList);
        self.vel += (accel_old + accel_exp) * 0.5 * dt;         // New vel
        self.pos = pos_old + (self.vel + vel_old) * 0.5 * dt;   // New pos
    }
}
