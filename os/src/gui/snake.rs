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
use virtio_drivers::InputEvent;
use super::{button::Button, Component, Graphics, Panel};
use virtio_input_decoder::{Decoder, Key, KeyType};


pub struct SnakeGame {
    inner: UPIntrFreeCell<SnakeInner>,
}

pub struct SnakeInner {
    graphic: Graphics,
    pub panel: Arc<Panel>,
    pub food: Button,
    pub snake: VecDeque<Button>,
    pub seeds: i32,
}

const blksize: i32 = 20;

lazy_static::lazy_static!{
    pub static ref SNAKEGAME: Arc<SnakeGame> = Arc::new(SnakeGame::new(
        Size::new(400, 400),
        Point::new(200, 100),
    ));
}

impl SnakeGame {
    pub fn new(
        size: Size,
        point: Point,
    ) -> Self {
        let mut panel = Arc::new(Panel::new(size, point));
        // 创建蛇的节点
        let mut snake_points: VecDeque<Button> = VecDeque::new();
        snake_points.push_back(Button::new(
            Size::new(20, 20),
            Point::new(0, 0),
            Some(panel.clone()),
            "#".to_string(),
        ));
        // 创建 food 节点
        let food_button = Button::new(
            Size::new(20, 20),
            // Point::new((size.width / 2) as i32, (size.height / 2) as i32),
            Point::new(40, 40),
            Some(panel.clone()),
            "@".to_string(),
        );
        Self {
            inner: unsafe {
                UPIntrFreeCell::new(SnakeInner {
                    graphic: Graphics {
                        size,
                        point,
                        drv: GPU_DEVICE.clone(),
                    },
                    panel: panel,
                    food: food_button,
                    snake: snake_points,
                    seeds: 1819i32,
                })
            },
        }
    }

    pub fn deal_input(&self, event: InputEvent) {
        let dtype = match Decoder::decode(
            event.event_type as usize,
            event.code as usize,
            event.value as usize,
        ) {
            Ok(dtype) => dtype,
            Err(_) => return,
        };
        match dtype {
            virtio_input_decoder::DecodeType::Key(key, r#type) => {
                if r#type == KeyType::Press {
                    match key.to_char() {
                        Ok(mut k) => {
                            self.move_to(k);
                        }
                        Err(_) => {}
                    }
                }
            }
            virtio_input_decoder::DecodeType::Mouse(mouse) => {
                // println!("{:?}", mouse);
            },
        }
    }

    pub fn move_to(&self, dirt: char) {
        // println!("head {:?}", self.get_head());
        let can_eat = self.can_eat();
        if can_eat {
            println!("eat");
            self.put_food();
        } else {
            let head_point = self.get_head();
            let (left, top, right, bottom) = self.get_range();
            let mut inner = self.inner.exclusive_access();
            let tail = inner.snake.pop_front().unwrap();
            match dirt {
                'w' => {
                    if head_point.y > top {
                        let p = Point::new(
                            head_point.x,
                            head_point.y - blksize
                        );
                        tail.move_to(p);
                    }
                    inner.snake.push_back(tail);
                },
                'a' => {
                    if head_point.x > left {
                        let p = Point::new(
                            head_point.x - blksize,
                            head_point.y
                        );
                        tail.move_to(p);
                    }
                    inner.snake.push_back(tail);
                },
                's' => {
                    if head_point.y + blksize < bottom {
                        let p = Point::new(
                            head_point.x,
                            head_point.y + blksize
                        );
                        tail.move_to(p);
                    }
                    inner.snake.push_back(tail);
                },
                'd' => {
                    if head_point.x + blksize < right {
                        let p = Point::new(
                            head_point.x + blksize,
                            head_point.y
                        );
                        tail.move_to(p);
                    }
                    inner.snake.push_back(tail);
                }
                _ => {}
            };
            drop(inner);
            self.paint();
        }
    }

    pub fn get_head(&self) -> Point {
        let inner = self.inner.exclusive_access();
        let len = inner.snake.len();
        let (_, point) = inner.snake[len - 1].bound();
        point
    }

    pub fn get_range(&self) -> (i32, i32, i32, i32) {
        let mut inner = self.inner.exclusive_access();
        let (size, point) = inner.panel.bound();
        (point.x, point.y, point.x + size.width as i32, point.y + size.height as i32)
    }

    pub fn put_food(&self) {
        let inner = self.inner.exclusive_access();
        let (panel_size, panel_point) = inner.panel.bound();
        let (_, food_point) = inner.food.bound();
        drop(inner);
        let point = loop {
            let random = self.gen_rand();
            let mut x = random % panel_size.width as i32;
            x = x - (x % blksize);
            let mut y = random % panel_size.height as i32;
            y = y - (y % blksize);
            let point = Point::new(
                (panel_point.x + x),
                (panel_point.y + y),
                // (((panel_point.x + 60) as i32)/blksize + 1) * blksize,
                // (((panel_point.y + 60) as i32)/blksize + 1) * blksize,
            );
            let inner = self.inner.exclusive_access();
            if point == food_point {
                continue;
            }
            if let flag = inner.snake.iter().find(|e| {
                let (_, p) = e.bound();
                p == point
            }).is_none() {
                drop(inner);
                break point;
            }
        };
        let inner = self.inner.exclusive_access();
        let panel = &inner.panel;
        let new_head = Button::new(
            Size::new(20, 20),
            Point::new(food_point.x - 200, food_point.y - 100),
            Some(panel.clone()),
            "#".to_string(),
        );
        drop(inner);
        let mut inner = self.inner.exclusive_access();
        inner.snake.push_back(new_head);
        inner.food.move_to(point);
        drop(inner);
        self.paint();
    }

    pub fn can_eat(&self) -> bool {
        let head_point = self.get_head();
        let inner = self.inner.exclusive_access();
        let (_, food_point) = inner.food.bound();
        if (head_point.x + blksize == food_point.x && head_point.y == food_point.y) || 
            (head_point.x == food_point.x && head_point.y + blksize == food_point.y) ||
            (head_point.x - blksize == food_point.x && head_point.y == food_point.y) ||
            (head_point.x == food_point.x && head_point.y - blksize == food_point.y) {
                return true;
        } else {
            return false;
        }
    }

    fn gen_rand(&self) -> i32 {
        let mut inner = self.inner.exclusive_access();
        let seed = (inner.seeds * inner.seeds) % i32::MAX;
        inner.seeds = seed;
        seed        
    }
}



impl Component for SnakeGame {
    fn paint(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.panel.paint();
        inner.food.paint();
        let len = inner.snake.len();
        drop(inner);
        for i in 0..len {
            let mut inner = self.inner.exclusive_access();
            let posi = &inner.snake[i];
            posi.paint();
            drop(inner);
        }
    }

    fn add(&self, comp: alloc::sync::Arc<dyn Component>) {
        unreachable!()
    }

    fn bound(
        &self,
    ) -> (
        embedded_graphics::prelude::Size,
        embedded_graphics::prelude::Point,
    ) {
        let inner = self.inner.exclusive_access();
        (inner.graphic.size, inner.graphic.point)
    }
}



