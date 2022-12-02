use wgpu::{CommandEncoder, include_wgsl, RenderPass, RenderPipeline, TextureView};
use wgpu_noboiler::{App, State};
use wgpu_noboiler::render_pass::RenderPassCreator;
use wgpu_noboiler::render_pipeline::RenderPipelineCreator;

fn main() {
    App::new().
        input(|_state, _window| false)
        .render(render)
        .render_pipeline(render_pipeline)
        .run();
}

fn render(state: &State, mut encoder: CommandEncoder, view: TextureView) {
    let mut render_pass: RenderPass = RenderPassCreator::new(&mut encoder, &view).into();

    render_pass.set_pipeline(state.render_pipelines.get(0).unwrap());
    render_pass.draw(0..3, 0..1);

    drop(render_pass);

    state.queue.submit(std::iter::once(encoder.finish()));
}

fn render_pipeline(state: &State, vec: &mut Vec<RenderPipeline>) {
    let render_pass_creator = RenderPipelineCreator::from_state(state, "examples/shader.wgsl");

    vec.push(render_pass_creator.into());
}