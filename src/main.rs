mod calculations;
mod camera;
mod mesh;
mod renderer;

use calculations::{Planet, RawPlanetData};
use camera::Camera;
use nalgebra::Vector3;
use rand::Rng;
use renderer::Renderer;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const PLANET_ARRAY_SIZE: usize = 128;

// Helper functions
fn other_planets(count: usize, index: usize, planets: &mut [Planet]) -> Vec<Planet> {
    let (pre_planets, post_planets_and_current) = planets.split_at_mut(index);
    let (this_planet, post_planets) = post_planets_and_current.split_first_mut().unwrap();
    let chain = pre_planets.iter().chain(post_planets.iter());
    chain.cloned().collect::<Vec<_>>()
}

fn planets_to_raw_data(planets: &[Planet]) -> [RawPlanetData; PLANET_ARRAY_SIZE] {
    planets
        .iter()
        .map(|x| x.to_raw_data())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Celestial Simulation")
        .build(&event_loop)
        .unwrap();

    let scene_info = SceneInfo {
        mouse_pos: [0.0; 2],
        resolution: [800.0; 2],
        delta_time: 1.0,
        planet_count: 1,
    };

    let mut camera = Camera::default();
    camera.pos = Vector3::new(0.0, 0.0, -5.0);
    camera.roll = std::f32::consts::FRAC_PI_6;

    // Random number generator
    let mut rng = rand::thread_rng();
    let planet_count = 5;
    let mut planets: [Planet; PLANET_ARRAY_SIZE] = [Planet::default(); PLANET_ARRAY_SIZE];

    (0..planet_count).for_each(|i| {
        planets[i] = match i {
            0 => Planet::new(
                10000.0,
                [0.0, 0.0, 0.0],
                35.0,
                [1.0, 132.0 / 255.0, 0.0],
            ),
            _ => Planet::new(
                rng.gen_range(5.0..15.0),
                [rng.gen_range(100.0..500.0), 0.0, 0.0],
                rng.gen_range(5.0..15.0),
                [
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                ],
            ),
        }
    });

    // Set centripetal acceleration after initialized
    /* for i in 0..planet_count {
        planets[i].set_init_velocity(&other_planets(planet_count, i, &mut planets));
    } */

    let mut renderer = Renderer::new(&window, &planets_to_raw_data(&planets), &camera, scene_info);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut mouse_r_pressed = false;
    let mut pmouse = PhysicalPosition::new(0.0_f64, 0.0); // Previous mouse position

    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );

    imgui.set_ini_filename(None);

    let mut last_frame_time = std::time::Instant::now();

    let mut current_resolution: [f32; 2] = window.inner_size().into();

    camera.set_sensitivity([-1.0, 1.0]);

    event_loop
        .run(|event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                renderer.resize(new_size);
                current_resolution = new_size.into();
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let current_frame_time = std::time::Instant::now();
                let delta_time = current_frame_time - last_frame_time;

                let raw_camera_data = camera.to_raw_data();

                renderer.queue.write_buffer(
                    &renderer.camera_buffer,
                    0,
                    bytemuck::cast_slice(&[raw_camera_data]),
                );

                let raw_planet_data = PlanetData {
                    planets: planets_to_raw_data(&planets),
                };

                /* renderer.queue.write_buffer(
                    &renderer.planet_buffer,
                    0,
                    bytemuck::cast_slice(&[raw_planet_data]),
                ); */

                let info = SceneInfo {
                    mouse_pos: pmouse.into(),
                    resolution: current_resolution,
                    delta_time: delta_time.as_micros() as f32,
                    planet_count: planet_count as u32,
                };
                renderer.queue.write_buffer(
                    &renderer.info_buffer,
                    0,
                    bytemuck::cast_slice(&[info]),
                );
                renderer.render();
                last_frame_time = current_frame_time;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        // button: MouseButton::Right,
                        state,
                        ..
                    },
                ..
            } => {
                mouse_r_pressed = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                if mouse_r_pressed {
                    let dp: [f64; 2] = [position.x - pmouse.x, position.y - pmouse.y];
                    camera.rotate_from_mouse(dp);
                }
                pmouse = position;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let dist = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(dist) => dist.y as f32,
                };
                camera.scroll(dist);
            }
            _ => {}
        })
        .unwrap();
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct SceneInfo {
    mouse_pos: [f32; 2],
    resolution: [f32; 2],
    delta_time: f32,
    planet_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct PlanetData {
    planets: [RawPlanetData; PLANET_ARRAY_SIZE],
}
