
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Point2d {
    pub xy: [f32;2],
}

impl Point2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            xy: [x,y],
        }
    }

    fn to_screen_space(&self, state: &crate::gpu::State) -> Self {
        Point2d {
            xy: [((self.xy[0] + 1.0)/2.0)*state.config.width as f32, ((self.xy[1] + 1.0)/2.0)*state.config.height as f32],
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Rectangle {
    topleft: Point2d,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            topleft: Point2d::new(x, y),
            width,
            height,
        }
    }

    fn to_screen_space(&self, state: &crate::gpu::State) -> Self {
        Rectangle {
            topleft: self.topleft.to_screen_space(state),
            width: ((self.width)/2.0)*state.config.width as f32,
            height: ((self.height)/2.0)*state.config.height as f32,
        }
    }
}