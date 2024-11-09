use wgpu::{Backends, Instance, InstanceDescriptor, InstanceFlags, RequestAdapterOptions};

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
}

impl<'a> Renderer<'a> {
    fn new(window: winit::window::Window) {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance
            .create_surface(&window)
            .expect("Failed to create surface");

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
    }
}
