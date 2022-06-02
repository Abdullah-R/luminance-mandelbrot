use mandelbrot::*;
use glium::{glutin::{self, dpi::PhysicalPosition}, Surface, uniform};
use std::fs::read_to_string;

fn main() {

    //initial opengl and context setup
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //shape of window (we are applying the shader to the whole window)
    let shape = vec![
        Vertex { position: [-1., -1.] },
        Vertex { position: [ 1.,  -1.] },
        Vertex { position: [ 1., 1.] },
        Vertex { position: [-1., 1.] },
        Vertex { position: [-1., -1.] },
    ];

    //vertices
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    //load shaders
    let filenames = [read_to_string("./src/vs.glsl").unwrap(), read_to_string("./src/fs.glsl").unwrap()];
    let vertex_shader_src = filenames[0].as_str();
    let fragment_shader_src = filenames[1].as_str();

    //prepare program and state variables
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut pressed = false;
    let mut pos = PhysicalPosition{x:0.0, y:0.0};
    let mut transform: [f32; 3] = [0.0, 0.0, 1.0];

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {trans: transform},
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
                glutin::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                    match state {
                        glutin::event::ElementState::Pressed => { pressed = true; },
                        glutin::event::ElementState::Released => { pressed = false; },
                    }
                    return;
                },
                glutin::event::WindowEvent::CursorMoved { device_id, position, modifiers } => {
                    if pressed {
                        transform[0] = transform[0] + ((pos.x - position.x)/500.0) as f32*transform[2];
                        transform[1] =  transform[1] + ((pos.y - position.y)/500.0) as f32*transform[2];
                    }

                    pos = position.clone();

                    return;
                },
                glutin::event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                    println!("{:?}",delta);
                    match delta {
                        glutin::event::MouseScrollDelta::LineDelta(_ , x ) => {
                            transform[2] = transform[2] + x/3.0*transform[2];
                            if transform[2] < 0.0 {transform[2] = 0.0};
                            println!("{}",transform[2]);
                        }
                        _ => { return; }
                    }
                },
                _ => return,
            },
            _ => (),
        }
    });
}
