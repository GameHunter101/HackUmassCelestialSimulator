use nalgebra::Vector3;

// Gravitational constant, can probably adjust
const GRAV: f32 = 6.67430;

#[derive(Clone, Copy)]
pub struct Planet {
    // Physical properties
    mass: f32,
    pos: Vector3<f32>,
    vel: Vector3<f32>,

    // Display properties
    //active: bool,
    radius: f32,
    //rot_vel: f32, // Angular velocity in rad/s
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            mass: Default::default(),
            pos: Default::default(),
            vel: Default::default(),
            radius: Default::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawPlanetData {
    pub pos: [f32; 3],
    pub padding: f32,
    pub radius: f32,
}

impl Planet {
    pub fn new(mass: f32, pos: [f32; 3], radius: f32, ) -> Self {
        Planet {
            mass: mass,
            pos: Vector3::from(pos),
            vel: Default::default(),

            //active: true,
            radius: radius,
            //rot_vel: f32, // Angular velocity in rad/s
        }
    }

    // Calculate acceleration
    fn calc_accel(&self, planet_list: &[Planet]) -> Vector3<f32> {
        // Make sure planet_list doesn't contain self
        let mut accel: Vector3<f32> = Vector3::zeros();
        for p in planet_list {
            let dr = p.pos - self.pos;
            accel += p.mass * dr / dr.magnitude().powi(3);
        }
        accel * GRAV
    }

    // Do the actual moving
    pub fn step(&mut self, planet_list: &[Planet], dt: f32) {
        let pos_old = self.pos;
        let vel_old = self.vel;

        let accel_old = self.calc_accel(planet_list);
        self.pos = self.pos + self.vel * dt; // Expected pos
        let accel_exp = self.calc_accel(planet_list);
        self.vel += (accel_old + accel_exp) * 0.5 * dt; // New vel
        self.pos = pos_old + (self.vel + vel_old) * 0.5 * dt; // New pos
    }

    // Attempt to set inital centripetal velocity for stable orbit
    // For simplicity, planet_list only contains the star
    pub fn set_init_velocity(&mut self, planet_list: &[Planet]) {
        let accel = self.calc_accel(planet_list);
        // Attempt to get a Star
        let star = &planet_list[0];

        let dr = star.pos - self.pos;
        let mag = f32::sqrt(accel.magnitude() * dr.magnitude());
        let uv = Vector3::z_axis().cross(&dr.normalize());

        self.vel = uv * mag;
    }

    pub fn to_raw_data(&self) -> RawPlanetData {
        RawPlanetData {
            pos: self.pos.into(),
            padding: 0.0,
            radius: self.radius,
        }
    }
}
