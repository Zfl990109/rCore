use alloc::{
    collections::VecDeque,
    string::{String, ToString},
    sync::Arc,
};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{Dimensions, Point, Primitive, RgbColor, Size},
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
    Drawable,
};

use crate::{drivers::GPU_DEVICE, sync::UPIntrFreeCell};

use super::{button::Button, Component, Graphics, Panel};

pub struct Window {
    inner: UPIntrFreeCell<WindowInner>,
}

pub struct WindowInner {  
    size: Size,
    point: Point,  
    titel: Option<String>,
    comps: VecDeque<Arc<dyn Component>>,
}

impl Window {
    pub fn new(
        size: Size,
        point: Point,
        titel: Option<String>,
    ) -> Self {
        Self {
            inner: unsafe {
                UPIntrFreeCell::new(WindowInner {
                    size,
                    point,
                    titel,
                    comps: VecDeque::new(),
                })
            },
        }
    }
}

impl Component for Window {
    fn paint(&self) {
        let mut inner = self.inner.exclusive_access();
        let len = inner.comps.len();
        drop(inner);
        for i in 0..len {
            let mut inner = self.inner.exclusive_access();
            let comp = Arc::downgrade(&inner.comps[i]);
            drop(inner);
            comp.upgrade().unwrap().paint();
        }
    }

    fn add(&self, comp: Arc<dyn Component>) {
        let mut inner = self.inner.exclusive_access();
        inner.comps.push_back(comp);
    }

    fn bound(&self) -> (Size, Point) {
        let mut inner = self.inner.exclusive_access();
        (inner.size, inner.point)
    }
}

