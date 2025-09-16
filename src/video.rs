use crate::draw::render::render;
use crate::prelude::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

pub struct Video {
    pub path: PathBuf,
    pub resolution: Resolution,
    pub fps: u32,
    draw_commands: DrawCommands,
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
            draw_commands: DrawCommands::new(),
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

    pub fn wait(&mut self, second: f32) {
        for (_, shape) in &self.shapes {
            self.draw_commands
                .commands
                .append(&mut shape.draw().commands)
        }

        self.draw_commands
            .commands
            .push(DrawCommand::Wait { second });
    }

    pub fn generate(self) {
        let start = Instant::now();

        let mut ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgba",
                "-s",
                &format!("{}x{}", self.resolution.width, self.resolution.height),
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

        let ffmpeg_stdin = ffmpeg.stdin.as_mut().unwrap();

        render(
            self.fps,
            self.resolution.width,
            self.resolution.height,
            self.draw_commands,
            ffmpeg_stdin,
        );

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
