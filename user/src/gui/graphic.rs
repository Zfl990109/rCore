use alloc::sync::Arc;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{OriginDimensions, Point, RgbColor, Size},
};


#[derive(Clone)]
pub struct Graphics {
    pub size: Size,
    pub point: Point,
}

impl Graphics {
    pub fn new(size: Size, point: Point) -> Self {
        Self {
            size,
            point,
        }
    }
}

impl OriginDimensions for Graphics {
    fn size(&self) -> Size {
        self.size
    }
}

