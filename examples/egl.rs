use glow::HasContext;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::GlDisplay;
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    //let window_attributes = winit::window::WindowAttributes::new();

    let template = ConfigTemplateBuilder::new();

    let display_builder =
        DisplayBuilder::new().with_preference(glutin_winit::ApiPreference::PreferEgl);

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .map(|c| dbg!(c))
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let gl_display = gl_config.display();
    /*assert!(gl_display
    .version_string()
    .to_ascii_lowercase()
    .contains("egl"));*/
    dbg!(gl_display.version_string());

    let not_current = unsafe {
        gl_display
            .create_context(&gl_config, &ContextAttributesBuilder::new().build(None))
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(
                        &gl_config,
                        &ContextAttributesBuilder::new()
                            .with_context_api(ContextApi::Gles(None))
                            .build(None),
                    )
                    .expect("failed to create context")
            })
    };

    // Make the context current
    let cx = match not_current {
        glutin::context::NotCurrentContext::Egl(cx) => cx,
        _ => panic!("Not expected"),
    };
    let _context = cx.make_current_surfaceless().unwrap();

    let gl =
        unsafe { glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s)) };
    println!("{:?}", gl.supported_extensions())
}
