use std::borrow::Cow;
use std::fs;

use wgpu::{RenderPipeline, ShaderModule, VertexBufferLayout};

use crate::State;

pub struct RenderPipelineCreator<'a> {
    state: &'a State,

    shader: ShaderModule,
    vertex_main: &'a str,
    fragment_main: &'a str,

    vertex_buffer: Vec<VertexBufferLayout<'a>>,

    label: &'a str,
}

impl<'a> RenderPipelineCreator<'a> {
    pub fn from_shader_file(path: &'a str, state: &'a State) -> RenderPipelineCreator<'a> {
        let string = fs::read_to_string(path).expect("TODO: panic message");

        let shader = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Pipeline Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::from(string)),
        });

        RenderPipelineCreator {
            state,
            shader,
            vertex_main: "vs_main",
            fragment_main: "fs_main",

            vertex_buffer: vec![],

            label: "Render Pipeline",
        }
    }

    pub fn add_vertex_buffer(mut self, buffer: VertexBufferLayout<'a>) -> Self {
        self.vertex_buffer.push(buffer);

        self
    }

    pub fn fragment_main(mut self, fn_name: &'a str) -> Self {
        self.fragment_main = fn_name;
        self
    }

    pub fn vertex_main(mut self, fn_name: &'a str) -> Self {
        self.vertex_main = fn_name;
        self
    }

    pub fn build(&self) -> RenderPipeline {
        let render_pipeline_layout = self.state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&(self.label.to_owned() + " Layout")),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        self.state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(self.label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: self.vertex_main,
                buffers: &self.vertex_buffer[..],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: self.fragment_main,
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.state.config.format,
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