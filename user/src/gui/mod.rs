mod button;
mod graphic;
mod panel;
use alloc::sync::Arc;
pub use button::*;
use core::any::Any;
use embedded_graphics::prelude::{Point, Size};
pub use graphic::*;
pub use panel::*;

pub trait Component: Send + Sync + Any {
    fn add(&mut self, comp: Arc<dyn Component>);
    fn bound(&self) -> (Size, Point);
}
