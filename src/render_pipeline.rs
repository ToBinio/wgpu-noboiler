use std::borrow::Cow;
use std::fs;

use wgpu::{BindGroupLayout, BlendState, ColorTargetState, ColorWrites, DepthStencilState, Device, Face, FragmentState, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexBufferLayout, VertexState};

use crate::app::AppData;

/// Builder Patter for wgpu [RenderPipeline]
pub struct RenderPipelineCreator<'a> {
    device: &'a Device,
    format: &'a TextureFormat,

    shader: ShaderModule,
    vertex_main: &'a str,
    fragment_main: &'a str,

    vertex_buffers: Vec<VertexBufferLayout<'a>>,
    bind_groups: Vec<&'a BindGroupLayout>,

    depth_stencil: Option<DepthStencilState>,

    label: &'a str,
}

impl<'a> RenderPipelineCreator<'a> {
    /// creates an [RenderPipelineCreator] where the shader is from the path
    pub fn from_shader_file(path: &'a str, app_data: &'a AppData) -> RenderPipelineCreator<'a> {
        let shader_code = fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not find Shader-File at {}", path));

        let shader = app_data.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Render Pipeline Shader"),
            source: ShaderSource::Wgsl(Cow::from(shader_code)),
        });

        RenderPipelineCreator {
            device: &app_data.device,
            format: &app_data.config.format,
            shader,
            vertex_main: "vs_main",
            fragment_main: "fs_main",

            vertex_buffers: vec![],
            bind_groups: vec![],

            depth_stencil: None,

            label: "Render Pipeline",
        }
    }

    /// adds a [VertexBufferLayout] to the used list
    pub fn add_vertex_buffer(mut self, layout: VertexBufferLayout<'a>) -> Self {
        self.vertex_buffers.push(layout);

        self
    }

    /// adds a [BindGroupLayout] to the used list
    pub fn add_bind_group(mut self, layout: &'a BindGroupLayout) -> Self {
        self.bind_groups.push(layout);

        self
    }

    /// sets the name of the Fragment-Main
    pub fn fragment_main(mut self, fn_name: &'a str) -> Self {
        self.fragment_main = fn_name;
        self
    }

    /// sets the name of the Vertex-Main
    pub fn vertex_main(mut self, fn_name: &'a str) -> Self {
        self.vertex_main = fn_name;
        self
    }

    /// sets the used [DepthStencilState]
    pub fn depth_stencil(mut self, depth_stencil: DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    /// creates a [RenderPipeline]
    pub fn build(&self) -> RenderPipeline {
        let render_pipeline_layout = self.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(&(self.label.to_owned() + " Layout")),
            bind_group_layouts: &self.bind_groups[..],
            push_constant_ranges: &[],
        });

        self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(self.label),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &self.shader,
                entry_point: self.vertex_main,
                buffers: &self.vertex_buffers[..],
            },
            fragment: Some(FragmentState {
                module: &self.shader,
                entry_point: self.fragment_main,
                targets: &[Some(ColorTargetState {
                    format: self.format.to_owned(),
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: self.depth_stencil.to_owned(),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}