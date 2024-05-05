use imgui_winit_support::winit::window::Window;
use imgui::*;


pub struct SafehouseDebugger {

}

impl SafehouseDebugger {
    pub fn new(window: &Window) -> Self {

        

        let mut context = imgui::Context::create();
        context.set_ini_filename(None);

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut context);
        platform.attach_window(context.io_mut(), window, imgui_winit_support::HiDpiMode::Default);

        let mut io = context.io_mut();

        context.fonts().add_font(&[
            FontSource::DefaultFontData { config: Some(imgui::FontConfig{
                size_pixels: 11.0,
                oversample_h: 1,
                pixel_snap_h: true,
                ..Default::default()
            }) }
        ]);

        SafehouseDebugger {}
        
    }

    pub fn debug(&self) {
        println!("Debugging...");
    }
}