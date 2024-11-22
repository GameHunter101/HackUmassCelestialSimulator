use nalgebra::Vector3;

// Gravitational constant, can probably adjust
const GRAV: f32 = 6.67430;

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    // Physical properties
    mass: f32,
    pub pos: Vector3<f32>,
    // Velocity used for orbital calculations
    pub vel: Vector3<f32>,
    // Vel used to apply 'forces', reset to vel after step
    pub vel2: Vector3<f32>,

    // Display properties
    //active: bool,
    radius: f32,
    //rot_vel: f32, // Angular velocity in rad/s
    color: [f32; 3],
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            mass: Default::default(),
            pos: Default::default(),
            vel: Default::default(),
            vel2: Default::default(),
            radius: Default::default(),
            color: [0.5; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawPlanetData {
    pub pos: [f32; 3],
    // pub padding: f32,
    pub radius: f32,
    pub color: [f32; 3],
    pub padding: f32,
}

impl Planet {
    pub fn new(mass: f32, pos: [f32; 3], radius: f32, color: [f32; 3]) -> Self {
        Planet {
            mass,
            pos: Vector3::from(pos),
            vel: Default::default(),
            vel2: Default::default(),

            //active: true,
            radius,
            color,
            //rot_vel: f32, // Angular velocity in rad/s
        }
    }

    // Calculate acceleration
    pub fn calc_accel(&self, planet_list: &mut [&mut Planet]) -> Vector3<f32> {
        // Make sure planet_list doesn't contain self
        let mut accel: Vector3<f32> = Vector3::zeros();
        for p in planet_list {
            let dr = p.pos - self.pos;
            accel += p.mass * dr / dr.magnitude().powi(3);
        }
        accel * GRAV
    }

    pub fn calc_collision(&mut self, planet_list: &mut [&mut Planet]) {
        for planet in planet_list {
            let delta_pos = self.pos - planet.pos;
            let collision_distance = planet.radius + self.radius;
            if delta_pos.magnitude() - collision_distance <= 0.0 {

                self.vel2 -= (2.0 * planet.mass) / (self.mass+planet.mass)
                    * (self.vel - planet.vel).dot(&delta_pos)
                    / delta_pos.norm_squared() * delta_pos;
            }
        }
    }

    // Do the actual moving
    pub fn step(&mut self, planet_list: &mut [&mut Planet], dt: f32) {
        // Apply external 'forces'
        self.vel = self.vel2;

        let pos_old = self.pos;
        // println!("Pos: {pos_old}");
        let vel_old = self.vel;

        let accel_old = self.calc_accel(planet_list);
        self.pos = self.pos + self.vel * dt; // Expected pos
        let accel_exp = self.calc_accel(planet_list);
        self.vel += (accel_old + accel_exp) * 0.5 * dt; // New vel
        self.pos = pos_old + (self.vel + vel_old) * 0.5 * dt; // New pos

        // Update vel2 to match
        self.vel2 = self.vel;
    }

    // Attempt to set inital centripetal velocity for stable orbit
    // For simplicity, planet_list only contains the star
    pub fn set_init_velocity(&mut self, planet_list: &mut [&mut Planet]) {
        let accel = self.calc_accel(planet_list);
        // println!("Accel: {accel}");
        // Attempt to get a Star
        let star = &planet_list[0];

        let dr = star.pos - self.pos;
        let mag = f32::sqrt(accel.magnitude() * dr.magnitude());
        let uv = Vector3::y_axis().cross(&dr.normalize());

        self.vel2 = uv * mag;
    }

    pub fn to_raw_data(&self) -> RawPlanetData {
        RawPlanetData {
            pos: self.pos.into(),
            // padding: 0.0,
            radius: self.radius,
            color: self.color,
            padding: 0.0,
        }
    }
}
