use imgui_wgpu::RendererConfig;
use safehouse_render::{self as render, gpu};
use render::gpu::winit::window::Window;

pub struct Debugger {
    imgui_context: imgui::Context,
    imgui_platform: imgui_winit_support::WinitPlatform,
    imgui_renderer: imgui_wgpu::Renderer,
}

// impl Debugger {
//     fn init(window: &Window, state: &gpu::State) -> Self {

//         let mut imgui_context = imgui::Context::create();
//         let mut imgui_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
//         imgui_platform.attach_window(
//             imgui_context.io_mut(), 
//             window, 
//             hidpi_mode
//         );

//         let rend_conf = RendererConfig {
//             texture_format: state.config.format.clone(),
//             ..Default::default()
//         };

//         Self {
//             imgui_context,
//             imgui_platform,
//             imgui_renderer: todo!(),
//         }
//     }

// }