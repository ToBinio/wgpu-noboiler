use bytemuck::Pod;
use wgpu::{Buffer, BufferUsages, Device};
use wgpu::util::DeviceExt;

pub struct BufferCreator<'a, T: Pod> {
    data: Vec<T>,
    device: &'a Device,
    usage: BufferUsages,

    label: Option<&'a str>,
}

impl<'a> BufferCreator<'a, i32> {
    pub fn indices(device: &'a Device) -> BufferCreator<'a, i32> {
        BufferCreator {
            data: vec![],
            device,
            usage: BufferUsages::INDEX,
            label: None,
        }
    }
}

impl<'a, T: Pod> BufferCreator<'a, T> {
    pub fn vertex(device: &'a Device) -> BufferCreator<'a, T> {
        BufferCreator {
            data: vec![],
            device,
            usage: BufferUsages::VERTEX,
            label: None,
        }
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn generate_buffer(&self) -> Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: self.label,
                contents: bytemuck::cast_slice(&self.data),
                usage: self.usage,
            }
        )
    }
}