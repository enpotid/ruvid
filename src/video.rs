use std::path::PathBuf;

pub struct Video {
    pub path: PathBuf,
    pub resolution: Resolution,
    pub fps: usize,
}

pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

impl Video {
    pub fn new(path: impl Into<PathBuf>, resolution: (usize, usize), fps: usize) -> Self {
        Video {
            path: path.into(),
            resolution: Resolution {
                width: resolution.0,
                height: resolution.1,
            },
            fps,
        }
    }
}
