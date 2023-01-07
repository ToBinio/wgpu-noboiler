use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, TextureView};

/// Builder Patter for wgpu [RenderPass]
pub struct RenderPassCreator<'a> {
    view: &'a TextureView,

    label: &'a str,

    clear_color: Color,

    depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,

    color_attachments: Vec<Option<RenderPassColorAttachment<'a>>>,
}

impl<'a> RenderPassCreator<'a> {
    pub fn new(view: &'a TextureView) -> RenderPassCreator<'a> {
        RenderPassCreator {
            view,
            label: "Render Pass",
            clear_color: Color::WHITE,
            depth_stencil_attachment: None,
            color_attachments: vec![],
        }
    }

    /// sets label (name)
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    /// sets clear_color (background color)
    pub fn clear_color(mut self, clear_color: Color) -> Self {
        self.clear_color = clear_color;
        self
    }

    /// sets the used [RenderPassDepthStencilAttachment]
    pub fn depth_stencil_attachment(mut self, depth_stencil_attachment: RenderPassDepthStencilAttachment<'a>) -> Self {
        self.depth_stencil_attachment = Some(depth_stencil_attachment);
        self
    }

    /// creates a [RenderPass]
    pub fn build(mut self, encoder: &'a mut CommandEncoder) -> RenderPass<'a> {
        self.color_attachments.push(Some(RenderPassColorAttachment {
            view: self.view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(self.clear_color),
                store: true,
            },
        }));

        let descriptor = RenderPassDescriptor {
            label: Some(self.label),
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: self.depth_stencil_attachment.clone(),
        };

        encoder.begin_render_pass(&descriptor)
    }

    /// returns the [RenderPassDescriptor]
    pub fn descriptor(&'a mut self) -> RenderPassDescriptor<'a, 'a> {
        self.color_attachments.push(Some(RenderPassColorAttachment {
            view: self.view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(self.clear_color),
                store: true,
            },
        }));

        RenderPassDescriptor {
            label: Some(self.label),
            color_attachments: &self.color_attachments,
            depth_stencil_attachment: self.depth_stencil_attachment.clone(),
        }
    }
}