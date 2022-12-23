use std::mem;

use wgpu::VertexAttribute;

pub trait Vertex<const SIZE: usize> {

    fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> where Self: Sized {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
    const ATTRIBS: [VertexAttribute; SIZE];
}