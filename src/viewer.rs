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

            if let Some(picture) = self.current_image().image() {
                let win_w = args.draw_width;
                let win_h = args.draw_height;

                let img_w = picture.width();
                let img_h = picture.height();

                let x_scale =
                    if img_w > win_w {
                        win_w as f64 / img_w as f64
                    } else  {
                        1.0
                    };

                let y_scale =
                    if img_h > win_h {
                        win_h as f64 / img_h as f64
                    } else  {
                        1.0
                    };

                let texture = opengl_graphics::Texture::from_image(
                    &picture.to_rgba(),
                    &texture::TextureSettings::new(),
                );

                graphics::image(
                    &texture,
                    c.transform.scale(x_scale, y_scale),
                    gl,
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
