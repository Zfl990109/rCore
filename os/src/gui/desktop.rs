
use alloc::{
    collections::VecDeque,
    string::{String, ToString},
    sync::Arc,
};
use embedded_graphics::{
    prelude::{Point, Size},
};
use crate::board::{VIRTGPU_XRES, VIRTGPU_YRES};
use crate::{
    fs::ROOT_INODE,
    gui::{Button, Component, IconController, ImageComp, Panel, Terminal, Window, SNAKEGAME},
    sync::UPIntrFreeCell,
    syscall::PAD,
};



lazy_static::lazy_static!{
    pub static ref DESKTOP: Arc<Desktop> = Arc::new(Desktop::new());
}

static DT: &[u8] = include_bytes!("../assert/desktop.bmp");

pub fn desktop_init() {
    let desktop = DESKTOP.clone();
    println!("desktop inited!");
}


pub struct Desktop {
    pub main_window: Arc<Window>,
    inner: UPIntrFreeCell<DesktopInner>,
}

pub struct DesktopInner {
    windows: VecDeque<Arc<Window>>,
}

impl Desktop {
    pub fn new() -> Self {
        let wd = Arc::new(Window::new(Size::new(VIRTGPU_XRES, VIRTGPU_YRES), Point::new(0, 0), Some("desktop".to_string())));
        let mut p: Arc<dyn Component + 'static> =
        Arc::new(Panel::new(Size::new(VIRTGPU_XRES, VIRTGPU_YRES), Point::new(0, 0)));
        let image = ImageComp::new(Size::new(VIRTGPU_XRES, VIRTGPU_YRES), Point::new(0, 0), DT, Some(p.clone()));
        let icon = IconController::new(ROOT_INODE.ls(), Some(p.clone()));
        p.add(Arc::new(image));
        p.add(Arc::new(icon));
        wd.add(p.clone());
        
        // let terminal_wd = Arc::new(Window::new(Size::new(400, 400), Point::new(200, 100), Some("terminal".to_string())));
        // let arc_t = Arc::new(Terminal::new(
        //     Size::new(400, 400),
        //     Point::new(200, 100),
        //     Some(terminal_wd.clone()),
        //     Some("demo.txt".to_string()),
        //     "".to_string(),
        // ));
        // let text = Panel::new(Size::new(400, 400), Point::new(200, 100));
        // let button = Button::new(
        //     Size::new(20, 20),
        //     Point::new(370, 10),
        //     Some(arc_t.clone()),
        //     "x".to_string(),
        // );
        // arc_t.add(Arc::new(text));
        // arc_t.add(Arc::new(button));
        // terminal_wd.add(arc_t.clone());
        // terminal_wd.paint();

        // let mut pad = PAD.exclusive_access();
        // *pad = Some(arc_t);

        


        let snake_wd = Arc::new(Window::new(Size::new(400, 400), Point::new(200, 100), Some("snake".to_string())));
        snake_wd.add(SNAKEGAME.clone());
        // snake_wd.paint();
        let dk = Desktop {
            main_window: snake_wd.clone(),
            inner: unsafe {
                UPIntrFreeCell::new(DesktopInner {
                    windows: VecDeque::new(),
                })
            },
        };
        let mut inner = dk.inner.exclusive_access();
        inner.windows.push_back(wd.clone());
        inner.windows.push_back(snake_wd.clone());
        wd.paint();
        snake_wd.paint();
        drop(inner);
        dk
    }
}




