use std::iter::once;
use std::thread;
use std::time::Duration;

use wgpu::{CommandEncoder, PresentMode, RenderPipeline, TextureView, VertexAttribute};
use winit::dpi::PhysicalSize;

use wgpu_noboiler::app::{AppCreator, AppData};
use wgpu_noboiler::buffer::BufferCreator;
use wgpu_noboiler::render_pass::RenderPassCreator;
use wgpu_noboiler::render_pipeline::RenderPipelineCreator;
use wgpu_noboiler::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ColoredPosVertex {
    position: [f32; 2],
}

impl Vertex<1> for ColoredPosVertex {
    const ATTRIBS: [VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];
}

struct State {
    pos: (f32, f32),
    vel: (f32, f32),
    x_scale: f32,
}

const TRIANGLE_SIZE: f32 = 0.3;

fn main() {
    AppCreator::new(State {
        pos: (0.5, 0.0),
        vel: (0.707, 0.707),
        x_scale: 9.0 / 16.0,
    })
    .init(init)
    .render(render)
    .update(update)
    .resize(resize)
    .present_mode(PresentMode::Immediate)
    .run()
}

fn render(app_data: &AppData, state: &mut State, mut encoder: CommandEncoder, view: TextureView) {
    let vertex_buffer = BufferCreator::vertex(&app_data.device)
        .data(vec![
            ColoredPosVertex {
                position: [
                    (0.0 + state.pos.0) * state.x_scale,
                    TRIANGLE_SIZE + state.pos.1,
                ],
            },
            ColoredPosVertex {
                position: [
                    (-TRIANGLE_SIZE + state.pos.0) * state.x_scale,
                    -TRIANGLE_SIZE + state.pos.1,
                ],
            },
            ColoredPosVertex {
                position: [
                    (TRIANGLE_SIZE + state.pos.0) * state.x_scale,
                    -TRIANGLE_SIZE + state.pos.1,
                ],
            },
        ])
        .build();

    {
        let mut render_pass = RenderPassCreator::new(&view).build(&mut encoder);

        render_pass.set_pipeline(app_data.render_pipelines.get(0).unwrap());
        render_pass.set_vertex_buffer(0, vertex_buffer.slice());

        render_pass.draw(0..3, 0..1);
    }

    app_data.queue.submit(once(encoder.finish()));
}

fn update(app_data: &AppData, state: &mut State) {
    if state.pos.0.abs() > 1.0 / state.x_scale - TRIANGLE_SIZE {
        state.vel.0 = -state.vel.0;
    }

    if state.pos.1.abs() > (1.0 - TRIANGLE_SIZE) {
        state.vel.1 = -state.vel.1;
    }

    state.pos.0 += state.vel.0 * app_data.delta_time as f32;
    state.pos.1 += state.vel.1 * app_data.delta_time as f32;
}

fn resize(_: &AppData, state: &mut State, size: &PhysicalSize<u32>) {
    state.x_scale = size.height as f32 / size.width as f32;
}

fn init(app_data: &AppData, _state: &mut State, vec: &mut Vec<RenderPipeline>) {
    let render_pipeline = RenderPipelineCreator::from_shader_file(
        "examples/shaderColorFromPos.wgsl",
        &app_data.device,
        &app_data.config,
    )
    .add_vertex_buffer(ColoredPosVertex::descriptor())
    .build();

    vec.push(render_pipeline);
}
