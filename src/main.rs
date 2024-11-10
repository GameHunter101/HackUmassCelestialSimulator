mod calculations;
mod camera;
mod mesh;
mod renderer;

use calculations::RawPlanetData;
use camera::Camera;
use nalgebra::Vector3;
use renderer::Renderer;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

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
        padding: 0.0,
    };

    let mut camera = Camera::default();
    camera.pos = Vector3::new(0.0, 0.0, -5.0);
    camera.roll = std::f32::consts::FRAC_PI_6;

    let mut planets = [RawPlanetData {
        mass: 10.0,
        pos: [20.0, 20.0, 0.0],
        padding: 0.0,
        radius: 10.0,
    }; 5];

    let mut renderer = Renderer::new(&window, &planets, &camera, scene_info);

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

                renderer.queue.write_buffer(
                    &renderer.planet_buffer,
                    0,
                    bytemuck::cast_slice(&[planets]),
                );

                let info = SceneInfo {
                    mouse_pos: pmouse.into(),
                    resolution: current_resolution,
                    delta_time: delta_time.as_micros() as f32,
                    padding: 0.0,
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
                event: WindowEvent::MouseWheel { delta: winit::event::MouseScrollDelta::LineDelta(x,y), .. },
                ..
            } => {
                camera.scroll(y);
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
    padding: f32,
}
