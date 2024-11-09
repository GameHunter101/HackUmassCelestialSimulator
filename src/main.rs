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

use imgui::*;
use imgui_wgpu::*;

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

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    let hidpi_factor = window.scale_factor();
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => elwt.exit(),

            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                renderer.resize(new_size);
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let now = std::time::Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
                renderer.render();
                
                // Create the UI
                platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    let ui = imgui.frame();

                    {
                        let window = ui.window("Hello world");
                        window
                            .size([300.0, 100.0], Condition::FirstUseEver)
                            .build(|| {
                                ui.text("Hello world!");
                                ui.text("This...is...imgui-rs on WGPU!");
                                ui.separator();
                                let mouse_pos = ui.io().mouse_pos;
                                ui.text(format!(
                                    "Mouse Position: ({:.1},{:.1})",
                                    mouse_pos[0], mouse_pos[1]
                                ));
                            });

                        let window = ui.window("Hello too");
                        window
                            .size([400.0, 200.0], Condition::FirstUseEver)
                            .position([400.0, 200.0], Condition::FirstUseEver)
                            .build(|| {
                                ui.text(format!("Frametime: who knows?"));
                            });
                    }
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

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct SceneInfo {
    mouse_pos: [f32; 2],
    resolution: [f32; 2],
    delta_time: f32,
    padding: f32,
}
