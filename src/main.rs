extern crate futures;
extern crate futures_spawn;
extern crate futures_threadpool;

extern crate image as image_rs;

extern crate piston;

extern crate piston_window;
use piston_window::*;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

extern crate sdl2_window;
use sdl2_window::Sdl2Window;

// extern crate image as image_rs;

use std::env::args_os;
use std::path::PathBuf;

mod picture;
mod viewer;

fn main() {
    // Replace this with clap eventually
    let images: Vec<PathBuf> = args_os().skip(1).map(|p| PathBuf::from(p)).collect();

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(
        "ImageViewer",
        [800, 600],
    )
    .opengl(opengl)
    // .fullscreen(true)
    .exit_on_esc(true)
    .build()
    .expect("Failed to create window");

    let mut viewer = viewer::Viewer::new(images);
    viewer.load_current_image();

    let mut gl = GlGraphics::new(opengl);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Render(args) => {
                viewer.render(&args, &mut gl);
            },
            Event::Update(_args) => {
                window.set_title(viewer.get_title());
                if viewer.should_close() {
                    window.set_should_close(true);
                }
            },
            Event::Input(input) => {
                viewer.input(&input);
            },
            _ => {},
        }
    }
}
