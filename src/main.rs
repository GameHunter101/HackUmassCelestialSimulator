mod camera;
mod renderer;
mod calculations;

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

    let mut renderer = Renderer::new(&window, &[12, 12, 12], &Camera::default());

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut mouseRPressed = false;
    let mut pmouse= PhysicalPosition::new(0.0_f64,0.0);  // Previous mouse position
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => elwt.exit(),

            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
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
                        // (camera).rotate_from_mouse(dp);
                    }

                    pmouse = position;
            }
            _ => {}
        })
        .unwrap();
}
