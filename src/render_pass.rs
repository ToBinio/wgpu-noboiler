use wgpu::{CommandEncoder, RenderPass, TextureView};

pub struct RenderPassCreator<'a> {
    encoder: &'a mut CommandEncoder,
    view: &'a TextureView,

    label: &'a str,
}

impl<'a> RenderPassCreator<'a> {
    pub fn new(encoder: &'a mut CommandEncoder, view: &'a TextureView) -> RenderPassCreator<'a> {
        RenderPassCreator {
            encoder,
            view,
            label: "Render Pass",
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn build(self) -> RenderPass<'a> {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(self.label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }
}