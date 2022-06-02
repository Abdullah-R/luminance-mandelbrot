use mandelbrot::*;
//use std::time::{Duration, Instant};
use glium::{glutin, Surface};
use std::fs::read_to_string;

fn main() {

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [-1., -1.] };
    let vertex2 = Vertex { position: [ 1.,  -1.] };
    let vertex3 = Vertex { position: [ 1., 1.] };
    let vertex4 = Vertex { position: [-1., 1.] };
    let vertex5 = Vertex { position: [-1., -1.] };

    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let filenames = [read_to_string("./src/vs.glsl").unwrap(), read_to_string("./src/fs.glsl").unwrap()];
    let vertex_shader_src = filenames[0].as_str();
    let fragment_shader_src = filenames[1].as_str();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
