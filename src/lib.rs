#[cfg(test)]
mod tests {
    #[test]
    fn dynamic_linkage() {
        use glow::HasContext;
        use glutin::api::egl::device::Device;
        use glutin::api::egl::display::Display;
        use glutin::config::ConfigSurfaceTypes;
        use glutin::{
            config::{ConfigTemplateBuilder, GlConfig},
            context::{ContextApi, ContextAttributesBuilder},
            display::GlDisplay,
        };
        let devices = Device::query_devices()
            .expect("Failed to query devices")
            .collect::<Vec<_>>();

        for (index, device) in devices.iter().enumerate() {
            println!(
                "Device {}: Name: {} Vendor: {}",
                index,
                device.name().unwrap_or("UNKNOWN"),
                device.vendor().unwrap_or("UNKNOWN")
            );
        }

        let device = devices.first().expect("No available devices");

        let display =
            unsafe { Display::with_device(device, None) }.expect("Failed to create display");

        let template = ConfigTemplateBuilder::default()
            .with_surface_type(ConfigSurfaceTypes::empty())
            .build();
        let config = unsafe { display.find_configs(template) }
            .unwrap()
            .reduce(|config, acc| {
                if config.num_samples() > acc.num_samples() {
                    config
                } else {
                    acc
                }
            })
            .expect("No available configs");

        println!("Picked a config with {} samples", config.num_samples());

        let not_current = unsafe {
            display
                .create_context(&config, &ContextAttributesBuilder::new().build(None))
                .unwrap_or_else(|_| {
                    display
                        .create_context(
                            &config,
                            &ContextAttributesBuilder::new()
                                .with_context_api(ContextApi::Gles(None))
                                .build(None),
                        )
                        .expect("failed to create context")
                })
        };

        // Make the context current for rendering
        let _context = not_current.make_current_surfaceless().unwrap();
        let gl =
            unsafe { glow::Context::from_loader_function_cstr(|s| display.get_proc_address(s)) };
        println!("{:?}", gl.supported_extensions())
    }
}
