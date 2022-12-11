pub mod render_pipeline;
pub mod render_pass;
pub mod buffer;
pub mod vertex;

use wgpu::{CommandEncoder, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration, SurfaceError, TextureView};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct State {
    surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    size: PhysicalSize<u32>,

    pub render_pipelines: Vec<RenderPipeline>,

    input_fn: Option<InputFn>,
    update_fn: Option<UpdateFn>,
    render_fn: Option<RenderFn>,
    render_pipeline_fn: Option<RenderPipelineFn>,
}

impl State {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        //todo call event
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn create_render_pipeline(&mut self) {
        match self.render_pipeline_fn {
            None => {}
            Some(function) => {
                let mut render_pipelines = Vec::new();

                function(self, &mut render_pipelines);

                self.render_pipelines = render_pipelines;
            }
        }
    }

    pub fn render(&self) -> Result<(), SurfaceError> {
        if self.render_fn.is_none() {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.render_fn.unwrap()(self, encoder, view);

        output.present();

        Ok(())
    }
}

pub struct App {
    state: State,

    window: Window,
    event_loop: EventLoop<()>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> App {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        )).unwrap();

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let state = State {
            surface,
            device,
            queue,
            config,
            size,

            render_pipelines: Vec::new(),

            input_fn: None,
            update_fn: None,
            render_fn: None,
            render_pipeline_fn: None,
        };

        App {
            state,

            window,
            event_loop,
        }
    }

    pub fn input(mut self, input: InputFn) -> Self {
        self.state.input_fn = Some(input);
        self
    }

    pub fn update(mut self, update: UpdateFn) -> Self {
        self.state.update_fn = Some(update);
        self
    }

    pub fn render(mut self, render: RenderFn) -> Self {
        self.state.render_fn = Some(render);
        self
    }

    pub fn render_pipeline(mut self, render_pipeline: RenderPipelineFn) -> Self {
        self.state.render_pipeline_fn = Some(render_pipeline);
        self
    }

    pub fn run(self) {
        let mut state = self.state;
        let window = self.window;

        state.create_render_pipeline();

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, window_id, } => {
                    if window_id == window.id() && !match state.input_fn {
                        None => false,
                        Some(function) => { function(&mut state, event) }
                    } {
                        match event {
                            WindowEvent::CloseRequested => {
                                *control_flow = ControlFlow::Exit
                            }
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id)
                if window_id == window.id() => {
                    match state.update_fn {
                        None => {}
                        Some(update) => {
                            update(&mut state);
                        }
                    }

                    match state.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => state.resize(state.size),
                        Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}

pub type InputFn = fn(_: &mut State, _: &WindowEvent) -> bool;

pub type UpdateFn = fn(_: &mut State);

pub type RenderFn = fn(_: &State, _: CommandEncoder, _: TextureView);

pub type RenderPipelineFn = fn(_: &State, _: &mut Vec<RenderPipeline>);