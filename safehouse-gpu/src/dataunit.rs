pub use image::ImageFormat;

pub enum UnitFormat {
    IMAGE(ImageFormat),
    FONT_TTF,
    VERTEXD,
    OBJ
}

pub type DataUnit<'a> = (&'static[u8], UnitFormat);

