use futures::Future;
use futures_spawn::SpawnHelper;
use futures_threadpool::ThreadPool;

use image_rs::GenericImage;

extern crate graphics;

// extern crate opengl_graphics;
use opengl_graphics::{self, GlGraphics};

// use piston_window::{Input, RenderArgs};
use piston_window::*;

use std::path::PathBuf;
use picture;

pub struct Viewer {
    index: usize,
    pictures: Vec<picture::Picture>,
    threadpool: ThreadPool,
}

impl Viewer {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Viewer {
            index: 0,
            pictures: paths.into_iter().map(|p| picture::Picture::new(p)).collect(),
            threadpool: ThreadPool::new(10),
        }
    }

    fn increment(&mut self) {
        let new_index = self.index.wrapping_add(1);
        self.index =
            if new_index >= self.pictures.len() {
                0
            } else {
                new_index
            };
        self.load_current_image();
    }

    fn decrement(&mut self) {
        let new_index = self.index.saturating_sub(1);
        self.index =
            if self.index == 0 {
                self.pictures.len() - 1
            } else {
                new_index
            };
        self.load_current_image();
    }

    pub fn load_current_image(&mut self) {
        if let None = self.current_image().image() {
            // self.threadpool.spawn({
                if let Err(e) = self.current_image_mut().load() {
                    println!("Failed to load {}: {}", self.current_image().path().display(), e);
                    self.pictures.remove(self.index);
                    if self.index > self.pictures.len() {
                        self.index = 0;
                    }
                    self.load_current_image();
                }
            // });
        }
    }

    fn current_image_mut(&mut self) -> &mut picture::Picture {
        &mut self.pictures[self.index]
    }

    fn current_image(&self) -> &picture::Picture {
        &self.pictures[self.index]
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        // let image = graphics::image::Image::new();

        gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);

            let win_w = args.draw_width;
            let win_h = args.draw_height;

            if let Some(picture) = self.current_image().image() {
                let img_w = picture.width();
                let img_h = picture.height();

                let img_ratio = img_w as f64 / img_h as f64;
                let win_ratio = win_w as f64 / win_h as f64;

                let scale = if win_ratio > img_ratio {
                    win_h as f64 / img_h as f64
                } else {
                    win_w as f64 / img_w as f64
                };

                let new_w = scale * img_w as f64;
                let new_h = scale * img_h as f64;

                let x_off = if new_w < win_w as f64 {
                    (win_w as f64 - new_w) / 2.0
                } else {
                    0.0
                };

                let y_off = if new_h < win_h as f64 {
                    (win_h as f64 - new_h) / 2.0
                } else {
                    0.0
                };

                let texture = opengl_graphics::Texture::from_image(
                    &picture.to_rgba(),
                    &texture::TextureSettings::new().convert_gamma(true),
                );

                graphics::image(
                    &texture,
                    c.transform.trans(x_off, y_off).scale(scale, scale),
                    gl,
                );
            } else {
                let black = rectangle::Rectangle::new([0.0, 0.0, 0.0, 1.0]);

                &black.draw(
                    [0.0, 0.0, win_w as f64, win_h as f64],
                    &c.draw_state,
                    c.transform,
                    gl
                );

            }
        });
    }

    pub fn get_title(&self) -> String {
        format!("ImageViewer - {}", &self.current_image().path().display())
    }

    pub fn should_close(&self) -> bool {
        self.pictures.len() == 0
    }

    pub fn input(&mut self, input: &Input) {
        match *input {
            Input::Release(button) => {
                match button {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Left => {
                                self.decrement();
                            },
                            Key::Right => {
                                self.increment();
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
