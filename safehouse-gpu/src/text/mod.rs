use glyphon::{Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};

pub struct TextRenderState {
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    cache: Cache,
    atlas: TextAtlas,
    renderer: TextRenderer,
    buffer: Buffer

}

impl TextRenderState {
    pub fn new(state: &crate::State, font_data: &'static [u8], text: &str) -> Self {

        let mut font_system = FontSystem::new();
        let mut swash_cache = SwashCache::new();
        let scale_factor = 1.0;
        let cache = Cache::new(&state.device);
        let mut viewport = Viewport::new(&state.device, &cache);
        let mut atlas = TextAtlas::new(&state.device, &state.queue, &cache, wgpu::TextureFormat::Bgra8UnormSrgb);
        let mut renderer = TextRenderer::new(&mut atlas, &state.device, crate::wgpu::MultisampleState::default(), None);
        let mut buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        let physical_width = (state.config.width as f64 * scale_factor) as f32;
        let physical_height = (state.config.height as f64 * scale_factor) as f32;
        buffer.set_size(&mut font_system, Some(physical_width), Some(physical_height));

        let mut trs = Self {
            font_system,
            swash_cache,
            viewport,
            cache,
            atlas,
            renderer,
            buffer,
        };

        trs.set_text(text);

        trs
    }

    pub fn resize(&mut self, physical_width: f32, physical_height: f32) {
        self.buffer.set_size(&mut self.font_system, Some(physical_width), Some(physical_height));
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer.set_text(&mut self.font_system, text, Attrs::new().family(Family::SansSerif), Shaping::Advanced);
        self.buffer.shape_until_scroll(&mut self.font_system, false);
    }

    pub fn prepare(&mut self, state: &crate::State) {
        // self.brush.queue::<&str>::(&state.device, &state.queue, vec![]).expect("Error occured while rendering text")
        self.viewport.update(&state.queue, Resolution{
            width: state.config.width,
            height: state.config.height,
        });
        self.renderer
            .prepare(
                &state.device,
                &state.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: 600,
                        bottom: 160,
                    },
                    default_color: Color::rgb(255, 0, 0),
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache
            )
            .unwrap();
    }

    pub fn render<'renderpass>(&'renderpass self, pass: &mut wgpu::RenderPass<'renderpass>) {
        self.renderer.render(&self.atlas, &self.viewport, pass).expect("Error while rendering text!");
    }

    pub fn trim(&mut self) {
        self.atlas.trim();
    }
}