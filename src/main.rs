mod calculations;
mod camera;
mod mesh;
mod renderer;

use calculations::RawPlanetData;
use camera::Camera;
use renderer::Renderer;
use winit::{
    event::{Event, WindowEvent},
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
        mouse_pos: [0.0;2],
        resolution: [800.0;2],
        delta_time: 1.0,
        padding: 0.0,
    };

    let mut renderer = Renderer::new(
        &window,
        &[RawPlanetData {
            mass: 10.0,
            pos: [20.0, 20.0, 0.0],
            padding: 0.0,
            radius: 10.0,
        }; 5],
        &Camera::default(),
        scene_info
    );

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                renderer.resize(new_size);
            }
            Event::AboutToWait => {
                renderer.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                renderer.render();
            }
            _ => {}
        })
        .unwrap();
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct SceneInfo {
    mouse_pos: [f32; 2],
    resolution: [f32; 2],
    delta_time: f32,
    padding: f32,
}
