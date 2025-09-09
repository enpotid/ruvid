use crate::prelude::*;

use glutin::{ContextBuilder, event_loop::EventLoop};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

pub struct Video {
    pub path: PathBuf,
    pub resolution: Resolution,
    pub fps: u32,
    shapes: HashMap<usize, Box<dyn Shape>>,
    next_id: usize,
}

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Video {
    pub fn new(path: impl Into<PathBuf>, resolution: (u32, u32), fps: u32) -> Self {
        Self {
            path: path.into(),
            resolution: Resolution {
                width: resolution.0,
                height: resolution.1,
            },
            fps,
            shapes: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) -> usize {
        let id = self.next_id;
        self.shapes.insert(id, shape);
        self.next_id += 1;
        id
    }

    pub fn edit_shape(&mut self, id: usize, function: &impl Fn(Box<dyn Shape>) -> Box<dyn Shape>) {
        if let Some(shape) = self.shapes.remove(&id) {
            self.shapes.insert(id, function(shape));
        }
    }

    pub fn generate(self) {
        let start = Instant::now();

        let (width, height) = (self.resolution.width, self.resolution.height);
        let el = EventLoop::new();
        let headless_context = ContextBuilder::new()
            .build_headless(&el, glutin::dpi::PhysicalSize::new(width, height))
            .unwrap();
        let _headless_context = unsafe { headless_context.make_current().unwrap() };

        let mut ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgba",
                "-s",
                &format!("{width}x{height}"),
                "-r",
                &format!("{}", self.fps),
                "-i",
                "-",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                &format!("{}", self.path.to_str().unwrap()),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let _ffmpeg_stdin = ffmpeg.stdin.as_mut().unwrap();

        // TODO

        drop(ffmpeg.stdin.take().unwrap());
        ffmpeg.wait().unwrap();

        let duration = start.elapsed().as_secs_f32();
        println!(
            "Generated! Time: {:02}:{:02}:{:02}.{:02}",
            duration as usize / 3600,
            duration as usize % 3600 / 60,
            duration as usize % 60,
            (duration * 100.0) as usize % 100,
        );
    }
}
