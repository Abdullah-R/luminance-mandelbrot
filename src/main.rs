use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::tess::Mode;
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::process::exit;
use std::time::Instant;
use luminance_mandelbrot::*;

const SIZE: i32 = 100000;

fn main() {
    let dim = WindowDim::Windowed { 
        width: 1200, 
        height: 900
    };

    let surface = GlfwSurface::new_gl33("Hello, World!", WindowOpt::default().set_dim(dim));

    match surface {
        Ok(surface) => {
            eprintln!("graphics surface created");
            main_loop(surface);
        }

        Err(e) => {
            eprintln!("cannot create graphics surface:\n{}", e);
            exit(1);
        }
    }
}

fn main_loop(mut surface: GlfwSurface) {
    let start_t = Instant::now();
    let mut ctxt = surface.context;
    let events = surface.events_rx;
    let back_buffer = ctxt.back_buffer().expect("back buffer");

    let mut verts = Vec::new();

    let ind = (SIZE as f32).sqrt() as i32;

    for i in 1..SIZE {

        let x:f32 = ((i%ind - ind/2) as f32)/((ind/2) as f32);
        let y:f32 = ((i/ind - ind/2) as f32)/((ind/2) as f32);
        let comp = CompNum::new(1.3*(x-0.5), y);
        let col = if comp.mandel() {[255,0,0]} else {[0,0,0]};

        verts.push(Vertex::new(
            VertexPosition::new([x, y]),
            VertexRGB::new(col)
        ));

    }


    // at the beginning of main_loop
    let triangle = ctxt
        .new_tess()
        .set_vertices(&*verts)
        .set_mode(Mode::Point)
        .build()
        .unwrap();

    let mut program = ctxt
        .new_shader_program::<VertexSemantics, (), ()>()
        .from_strings(VS_STR, None, None, FS_STR)
        .unwrap()
        .ignore_warnings();
    
    'app: loop {
        // handle events
        ctxt.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break,
                _ => ()
            }
        }

        //rendering 
        //let t = start_t.elapsed().as_secs_f32();
        let color = [0., 0., 0., 1.];

        let render = ctxt
        .new_pipeline_gate()
        .pipeline(
            &back_buffer,
            &PipelineState::default().set_clear_color(color),
            |_ , mut shd_gate| {
                shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
                    rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                        tess_gate.render(&triangle)
                    })
                })
            }
        )
        .assume();

        // swap buffer chains
        if render.is_ok() {
            ctxt.window.swap_buffers();
        } else {
            break 'app;
        }
    }
}