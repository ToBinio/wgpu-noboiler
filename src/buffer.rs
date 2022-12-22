use bytemuck::Pod;
use wgpu::{Buffer, BufferSlice, BufferUsages, Device};
use wgpu::util::DeviceExt;

/// Builder Patter for wgpu Buffer
pub struct BufferCreator<'a, T: Pod> {
    data: Vec<T>,
    device: &'a Device,
    usage: BufferUsages,

    label: &'a str,
}

impl<'a> BufferCreator<'a, i32> {
    /// Helps to create indices-buffer
    ///
    /// IndexFormat: u32
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
    /// Helps to create vertex-buffer
    pub fn vertex(device: &'a Device) -> BufferCreator<'a, T> {
        BufferCreator {
            data: vec![],
            device,
            usage: BufferUsages::VERTEX,
            label: "Vertex Buffer",
        }
    }

    /// sets buffer Label (name of the buffer)
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;

        self
    }

    /// sets data of the Buffer
    ///
    /// overwrites data which was stored before
    pub fn data(mut self, data: Vec<T>) -> Self {
        self.data = data;
        self
    }

    /// push data to current dataVec
    pub fn add_data(&mut self, data: T) -> &mut Self {
        self.data.push(data);

        self
    }

    /// creates SimpleBuffer
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

/// wrapper for Buffer which stores count of elements
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