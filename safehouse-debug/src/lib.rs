use imgui_winit_support::winit::window::Window;


pub struct SafehouseDebugger {

}

impl SafehouseDebugger {
    pub fn new(window: &Window) -> Self {

        

        let mut context = imgui::Context::create();
        context.set_ini_filename(None);

        let mut platform = imgui_winit_support::WinitPlatform::init(&mut context);
        platform.attach_window(context.io_mut(), window, imgui_winit_support::HiDpiMode::Default);

        SafehouseDebugger {}
        
    }

    pub fn debug(&self) {
        println!("Debugging...");
    }
}