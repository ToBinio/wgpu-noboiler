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
    App::new().
        input(|_state, _window| false)
        .render(render)
        .render_pipeline(render_pipeline)
        .run();
}

fn render(state: &State, mut encoder: CommandEncoder, view: TextureView) {
    let mut vertex_creator = BufferCreator::vertex(&state.device);
    vertex_creator.set_data(vec![
        ColoredVertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        ColoredVertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        ColoredVertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        ColoredVertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        ColoredVertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
    ]);
    let vertex_buffer = vertex_creator.generate_buffer();

    let mut indices_creator = BufferCreator::indices(&state.device);
    indices_creator.set_data(vec![0, 1, 4,
                                  1, 2, 4,
                                  2, 3, 4, ]);

    let indices_buffer = indices_creator.generate_buffer();

    {
        let mut render_pass: RenderPass = RenderPassCreator::new(&mut encoder, &view).into();

        render_pass.set_pipeline(state.render_pipelines.get(0).unwrap());

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(indices_buffer.slice(..), wgpu::IndexFormat::Uint32);

        render_pass.draw_indexed(0..indices_creator.size() as u32, 0, 0..1);
    }

    state.queue.submit(std::iter::once(encoder.finish()));
}

fn render_pipeline(state: &State, vec: &mut Vec<RenderPipeline>) {
    let mut render_pipeline_creator = RenderPipelineCreator::from_state(state, "examples/shader.wgsl");
    render_pipeline_creator.add_vertex_buffer(ColoredVertex::descriptor());

    vec.push(render_pipeline_creator.into());
}