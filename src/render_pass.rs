use wgpu::{CommandEncoder, RenderPass, Texture, TextureView};

pub struct RenderPassCreator<'a> {
    encoder: &'a mut CommandEncoder,
    view: &'a TextureView,
}

impl<'a> RenderPassCreator<'a> {
    pub fn new(encoder: &'a mut CommandEncoder, view: &'a TextureView) -> RenderPassCreator<'a> {
        RenderPassCreator {
            encoder,
            view,
        }
    }
}

impl<'a> Into<RenderPass<'a>> for RenderPassCreator<'a> {
    fn into(self) -> RenderPass<'a> {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
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