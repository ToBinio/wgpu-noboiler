use bytemuck::Pod;
use wgpu::{Buffer, BufferSlice, BufferUsages, Device};
use wgpu::util::DeviceExt;

pub struct BufferCreator<'a, T: Pod> {
    data: Vec<T>,
    device: &'a Device,
    usage: BufferUsages,

    label: &'a str,
}

impl<'a> BufferCreator<'a, i32> {
    pub fn indices(device: &'a Device) -> BufferCreator<'a, i32> {
        BufferCreator {
            data: vec![],
            device,
            usage: BufferUsages::INDEX,
            label: "Indices Buffer",
        }
    }
}

impl<'a, T: Pod> BufferCreator<'a, T> {
    pub fn vertex(device: &'a Device) -> BufferCreator<'a, T> {
        BufferCreator {
            data: vec![],
            device,
            usage: BufferUsages::VERTEX,
            label: "Vertex Buffer",
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;

        self
    }

    pub fn data(mut self, data: Vec<T>) ->  Self {
        self.data = data;
        self
    }

    pub fn build(&self) -> SimpleBuffer {
        SimpleBuffer {
            buffer: self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(self.label),
                    contents: bytemuck::cast_slice(&self.data),
                    usage: self.usage,
                }
            ),

            size: self.data.len() as u32,
        }
    }
}

pub struct SimpleBuffer {
    buffer: Buffer,
    size: u32,
}

impl SimpleBuffer {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn slice(&self) -> BufferSlice {
        self.buffer.slice(..)
    }
}