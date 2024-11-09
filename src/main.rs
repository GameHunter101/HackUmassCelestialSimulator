mod camera;
mod renderer;
mod calculations;

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

    let mut renderer = Renderer::new(&window, &[12, 12, 12], &Camera::default());

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
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
