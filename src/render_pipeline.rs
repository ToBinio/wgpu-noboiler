use std::borrow::Cow;
use std::fs;
use wgpu::{Device, RenderPipeline, SurfaceConfiguration};
use crate::State;

pub struct RenderPipelineCreator<'a> {
    device: &'a Device,
    config: &'a SurfaceConfiguration,

    shader_path: &'a str,
}

impl<'a> RenderPipelineCreator<'a> {
    pub fn from_state(state: &'a State, shader_file_path: &'a str) -> RenderPipelineCreator<'a> {
        RenderPipelineCreator {
            device: &state.device,
            config: &state.config,
            shader_path: shader_file_path,
        }
    }
}

impl<'a> Into<RenderPipeline> for RenderPipelineCreator<'a> {
    fn into(self) -> RenderPipeline {
        let string = fs::read_to_string(self.shader_path).expect("TODO: panic message");

        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::from(string)).into(),
        });

        let render_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.config.format,
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
        });

        render_pipeline
    }
}