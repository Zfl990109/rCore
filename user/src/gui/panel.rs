use alloc::{collections::VecDeque, rc::Weak, sync::Arc};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
    Drawable,
};


use super::{Component, Graphics};

pub struct Panel {
    graphic: Graphics,
    comps: VecDeque<Arc<dyn Component>>,
}


impl Panel {
    pub fn new(size: Size, point: Point) -> Self {
        Self {
            graphic: Graphics {
                size,
                point,
            },
            comps: VecDeque::new(),
        }
    }
}

impl Component for Panel {

    fn add(&mut self, comp: alloc::sync::Arc<dyn Component>) {
        self.comps.push_back(comp);
    }

    fn bound(&self) -> (Size, Point) {
        (self.graphic.size, self.graphic.point)
    }
}
