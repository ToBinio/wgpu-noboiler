use std::borrow::Cow;
use std::fs;

use wgpu::{Device, RenderPipeline, SurfaceConfiguration, VertexBufferLayout};

use crate::State;

pub struct RenderPipelineCreator<'a> {
    device: &'a Device,
    config: &'a SurfaceConfiguration,

    shader_path: &'a str,

    vertex_buffer: Vec<VertexBufferLayout<'a>>,
}

impl<'a> RenderPipelineCreator<'a> {
    pub fn from_state(state: &'a State, shader_file_path: &'a str) -> RenderPipelineCreator<'a> {
        RenderPipelineCreator {
            device: &state.device,
            config: &state.config,
            shader_path: shader_file_path,
            vertex_buffer: vec![],
        }
    }

    pub fn add_vertex_buffer(&mut self, buffer: VertexBufferLayout<'a>) -> &mut Self {
        self.vertex_buffer.push(buffer);

        self
    }
}

impl<'a> From<RenderPipelineCreator<'a>> for RenderPipeline {
    fn from(render_pipeline_creator: RenderPipelineCreator) -> Self {
        let string = fs::read_to_string(render_pipeline_creator.shader_path).expect("TODO: panic message");

        let shader = render_pipeline_creator.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::from(string)),
        });

        let render_pipeline_layout = render_pipeline_creator.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        render_pipeline_creator.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &render_pipeline_creator.vertex_buffer[..],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: render_pipeline_creator.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}