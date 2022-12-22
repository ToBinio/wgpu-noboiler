use wgpu::{Color, CommandEncoder, RenderPass, TextureView};

pub struct RenderPassCreator<'a> {
    encoder: &'a mut CommandEncoder,
    view: &'a TextureView,

    label: &'a str,

    clear_color: Color,
}

impl<'a> RenderPassCreator<'a> {
    pub fn new(encoder: &'a mut CommandEncoder, view: &'a TextureView) -> RenderPassCreator<'a> {
        RenderPassCreator {
            encoder,
            view,
            label: "Render Pass",
            clear_color: Color::WHITE,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn clear_color(mut self, clear_color: Color) -> Self {
        self.clear_color = clear_color;
        self
    }

    pub fn build(self) -> RenderPass<'a> {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(self.label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }
}