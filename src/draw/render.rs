use crate::prelude::*;
use glutin::{ContextBuilder, event_loop::EventLoop};
use std::ffi::CString;
use std::io::Write;
use std::process::ChildStdin;
use std::ptr;

pub fn render(
    fps: u32,
    width: u32,
    height: u32,
    commands: DrawCommands,
    ffmpeg_stdin: &mut ChildStdin,
) {
    let el = EventLoop::new();
    let headless_context = ContextBuilder::new()
        .build_headless(&el, glutin::dpi::PhysicalSize::new(width, height))
        .unwrap();
    let headless_context = unsafe { headless_context.make_current().unwrap() };

    unsafe {
        gl::load_with(|s| headless_context.get_proc_address(s) as *const _);
        gl::Viewport(0, 0, width as i32, height as i32);
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.1, 0.2, 0.3, 1.0);

        let vertex_shader_src = r#"
               #version 330 core
               layout (location = 0) in vec3 aPos;
               void main() {
                   gl_Position = vec4(aPos, 1.0);
               }
           "#;

        let fragment_shader_src = r#"
               #version 330 core
               out vec4 FragColor;
               void main() {
                   FragColor = vec4(1.0, 0.5, 0.2, 1.0);
               }
           "#;

        let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
        let shader_program = link_program(vertex_shader, fragment_shader);

        let mut vbo = 0;
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (9 * std::mem::size_of::<f32>()) as isize,
            ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        let mut flipped_pixels = vec![0u8; (width * height * 4) as usize];
        for command in commands.commands {
            match command {
                DrawCommand::Wait { second } => {
                    for _ in 0..(second * fps as f32) as u32 {
                        ffmpeg_stdin.write_all(&flipped_pixels).unwrap();
                    }
                }
                DrawCommand::Line { p1, p2 } => {}
                DrawCommand::Triangle { p1, p2, p3 } => {
                    let vertices: [f32; 9] = [
                        2.0 * p1.x / width as f32,
                        2.0 * p1.y / height as f32,
                        0.0,
                        2.0 * p2.x / width as f32,
                        2.0 * p2.y / height as f32,
                        0.0,
                        2.0 * p3.x / width as f32,
                        2.0 * p3.y / height as f32,
                        0.0,
                    ];

                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        (vertices.len() * std::mem::size_of::<f32>()) as isize,
                        vertices.as_ptr() as *const _,
                    );

                    gl::UseProgram(shader_program);
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);

                    let mut pixels = vec![0u8; (width * height * 4) as usize];
                    gl::ReadPixels(
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        pixels.as_mut_ptr() as *mut _,
                    );

                    let row_bytes = (width * 4) as usize;
                    for y in 0..height as usize {
                        let src = y * row_bytes;
                        let dst = (height as usize - 1 - y) * row_bytes;
                        flipped_pixels[dst..dst + row_bytes]
                            .copy_from_slice(&pixels[src..src + row_bytes]);
                    }
                }
                _ => {}
            }
        }
    }
}

fn compile_shader(src: &str, ty: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();

        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as i32;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != (gl::TRUE as i32) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut _);
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        shader
    }
}

fn link_program(vs: u32, fs: u32) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as i32;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != (gl::TRUE as i32) {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut _);
            panic!(
                "Program linking failed: {}",
                String::from_utf8_lossy(&buffer)
            );
        }

        program
    }
}
