mod calculations;
mod camera;
mod mesh;
mod renderer;

use calculations::RawPlanetData;
use camera::Camera;
use renderer::Renderer;
use winit::{
    dpi::PhysicalPosition, event::{ElementState, Event, MouseButton, WindowEvent}, event_loop::EventLoop, window::WindowBuilder
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

    let mut camera = Camera::default();
    camera.set_sensitivity([-1.0,1.0]);
    let mut renderer = Renderer::new(
        &window,
        &[RawPlanetData {
            mass: 10.0,
            pos: [20.0, 20.0, 0.0],
            padding: 0.0,
            radius: 10.0,
        }; 5],
        &camera,
        scene_info
    );

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut mouseRPressed = false;
    // Previous mouse position
    let mut pmouse= PhysicalPosition::new(0.0_f64,0.0);
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
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                renderer.render();
            }
            Event::WindowEvent { event: WindowEvent::MouseInput 
                { button: MouseButton::Right, state, .. }, .. } => {
                mouseRPressed = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false
                };
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved
                { position, .. }, ..} => {
                    if (mouseRPressed) {
                        let dp: [f64;2] = [position.x - pmouse.x, position.y - pmouse.y];
                        camera.rotate_from_mouse(dp);
                    }

                    pmouse = position;
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
