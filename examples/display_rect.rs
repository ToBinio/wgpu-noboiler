use wgpu::{CommandEncoder, RenderPass, RenderPipeline, TextureView, VertexAttribute};

use wgpu_noboiler::{App, State};
use wgpu_noboiler::buffer::BufferCreator;
use wgpu_noboiler::render_pass::RenderPassCreator;
use wgpu_noboiler::render_pipeline::RenderPipelineCreator;
use wgpu_noboiler::vertex::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ColoredVertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex<2> for ColoredVertex {
    const ATTRIBS: [VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}

fn main() {
    App::new()
        .init_render_pipeline(render_pipeline)
        .render(render)
        .run();
}

fn render(state: &State, mut encoder: CommandEncoder, view: TextureView) {
    let vertex_buffer = BufferCreator::vertex(&state.device)
        .data(vec![
            ColoredVertex { position: [-0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
            ColoredVertex { position: [-0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
            ColoredVertex { position: [0.5, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
            ColoredVertex { position: [0.5, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
        ]).build();

    let indices_buffer = BufferCreator::indices(&state.device)
        .data(vec![0, 1, 2, 2, 1, 3]).build();

    {
        let mut render_pass: RenderPass = RenderPassCreator::new(&mut encoder, &view).build();

        render_pass.set_pipeline(state.render_pipelines.get(0).unwrap());

        render_pass.set_vertex_buffer(0, vertex_buffer.slice());
        render_pass.set_index_buffer(indices_buffer.slice(), wgpu::IndexFormat::Uint32);

        render_pass.draw_indexed(0..indices_buffer.size(), 0, 0..1);
    }

    state.queue.submit(std::iter::once(encoder.finish()));
}

fn render_pipeline(state: &State, vec: &mut Vec<RenderPipeline>) {
    let render_pipeline = RenderPipelineCreator::from_shader_file("examples/shader.wgsl", state)
        .add_vertex_buffer(ColoredVertex::descriptor())
        .build();

    vec.push(render_pipeline);
}