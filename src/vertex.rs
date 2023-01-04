use std::mem;

use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

pub trait Vertex<const SIZE: usize> {
    fn descriptor<'a>() -> VertexBufferLayout<'a> where Self: Sized {
        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: Self::STEP_MODE,
            attributes: &Self::ATTRIBS,
        }
    }

    const STEP_MODE: VertexStepMode = VertexStepMode::Vertex;
    const ATTRIBS: [VertexAttribute; SIZE];
}