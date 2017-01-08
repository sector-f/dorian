extern crate image;
use std::path::{Path, PathBuf};
use std::fmt::{self, Display};

pub struct Picture {
    path: PathBuf,
    image: Option<image::DynamicImage>,
}

impl Picture {
    pub fn new(path: PathBuf) -> Self {
        Picture {
            path: path,
            image: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn image(&self) -> Option<&image::DynamicImage> {
        self.image.as_ref()
    }

    pub fn load(&mut self) -> Result<(), image::ImageError> {
        if self.image.is_none() {
            self.image = Some(image::open(&self.path)?);
        }
        Ok(())
    }

    pub fn unload(&mut self) {
        self.image = None;
    }
}

impl Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path: {}\nLoaded: {}", self.path.display(), self.image.is_some())
    }
}
