use std::time::Instant;

use wgpu::{Backends, CommandEncoder, CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Instance, Limits, PowerPreference, PresentMode, Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError, TextureUsages, TextureView, TextureViewDescriptor};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct App<T: 'static> {
    state: T,
    app_data: AppData,

    window_event_fn: Option<WindowEventFn<T>>,
    resize_fn: Option<ResizeFn<T>>,
    update_fn: Option<UpdateFn<T>>,
    render_fn: Option<RenderFn<T>>,
    init_fn: Option<InitFn<T>>,
}

/// background data for your [App]
pub struct AppData {
    last_frame_instant: Instant,

    ///Avg Frames Per Seconds of the last 30 Frames
    pub fps: f64,
    ///time since the last frame in seconds
    pub delta_time: f64,

    surface: Surface,

    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,

    pub size: PhysicalSize<u32>,

    ///vec of all RenderPipelines which got [created](AppCreator::init)
    pub render_pipelines: Vec<RenderPipeline>,
}

impl<T: 'static> App<T> {
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 { return; }

        self.app_data.size = new_size;
        self.app_data.config.width = new_size.width;
        self.app_data.config.height = new_size.height;
        self.app_data.surface.configure(&self.app_data.device, &self.app_data.config);

        if self.resize_fn.is_some() {
            self.resize_fn.unwrap()(&self.app_data, &mut self.state, &self.app_data.size)
        }
    }

    fn init(&mut self) {
        if self.init_fn.is_none() { return; }

        let mut render_pipelines = Vec::new();
        self.init_fn.unwrap()(&self.app_data, &mut self.state, &mut render_pipelines);
        self.app_data.render_pipelines = render_pipelines
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        if self.render_fn.is_none() {
            return Ok(());
        }

        let output = self.app_data.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let encoder = self.app_data.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.render_fn.unwrap()(&self.app_data, &mut self.state, encoder, view);

        output.present();

        Ok(())
    }

    fn run(mut self, window: Window, event_loop: EventLoop<()>) {
        self.init();

        window.set_visible(true);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, window_id, } => {
                    if window_id != window.id() {
                        return;
                    }
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.resize(**new_inner_size);
                        }
                        _ => {
                            if self.window_event_fn.is_some() {
                                self.window_event_fn.unwrap()(&self.app_data, &mut self.state, event);
                            }
                        }
                    }
                }
                Event::RedrawRequested(window_id) => {
                    if window_id != window.id() { return; }

                    self.app_data.delta_time = self.app_data.last_frame_instant.elapsed().as_secs_f64();
                    self.app_data.fps = 1.0 / self.app_data.delta_time;

                    self.app_data.last_frame_instant = Instant::now();

                    if self.update_fn.is_some() {
                        self.update_fn.unwrap()(&self.app_data, &mut self.state);
                    }

                    match self.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => self.resize(self.app_data.size),
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

/// builder for [App]
pub struct AppCreator<T: 'static> {
    state: T,

    window: Window,
    event_loop: EventLoop<()>,

    window_event_fn: Option<WindowEventFn<T>>,
    resize_fn: Option<ResizeFn<T>>,
    update_fn: Option<UpdateFn<T>>,
    render_fn: Option<RenderFn<T>>,
    init_fn: Option<InitFn<T>>,

    present_mode: PresentMode,
}

impl<T: 'static> AppCreator<T> {
    /// creates [AppCreator]
    ///
    /// # Arguments
    ///
    /// * `state`: data which describes your App can be changes on [update](AppCreator::update) and used to [render](AppCreator::render)
    ///
    pub fn new(state: T) -> AppCreator<T> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        AppCreator {
            state,

            window,
            event_loop,
            window_event_fn: None,
            resize_fn: None,
            update_fn: None,
            render_fn: None,
            init_fn: None,

            present_mode: PresentMode::Fifo,
        }
    }

    /// gets called on every [WindowEvent]
    ///
    /// e.g. mouse | keyboard input
    pub fn window_event(mut self, input: WindowEventFn<T>) -> Self {
        self.window_event_fn = Some(input);
        self
    }

    /// gets called on every ResizeEvent
    pub fn resize(mut self, resize: ResizeFn<T>) -> Self {
        self.resize_fn = Some(resize);
        self
    }

    ///gets called on every frame just before [AppCreator::render]
    ///
    /// here you can change your State
    pub fn update(mut self, update: UpdateFn<T>) -> Self {
        self.update_fn = Some(update);
        self
    }

    ///gets called on every frame just after [AppCreator::update]
    ///
    /// here you can render your frame
    pub fn render(mut self, render: RenderFn<T>) -> Self {
        self.render_fn = Some(render);
        self
    }

    /// gets called before opening the window
    ///
    /// mainly used to create your [RenderPipelines](RenderPipeline)
    pub fn init(mut self, init: InitFn<T>) -> Self {
        self.init_fn = Some(init);
        self
    }

    /// sets the [PresentMode] of the [Surface]
    pub fn present_mode(mut self, present_mode: PresentMode) -> Self {
        self.present_mode = present_mode;
        self
    }

    pub fn title(self, title: &str) -> Self {
        self.window.set_title(title);
        self
    }

    pub fn resizable(self, resizable: bool) -> Self {
        self.window.set_resizable(resizable);
        self
    }

    pub fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn create_app_data(&self) -> AppData {
        env_logger::init();
        let size = self.window.inner_size();

        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(&self.window) };

        let adapter = pollster::block_on(instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    Limits::downlevel_webgl2_defaults()
                } else {
                    Limits::default()
                },
                label: None,
            },
            None,
        )).unwrap();

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: self.present_mode,
            alpha_mode: CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        AppData {
            surface,
            device,
            queue,
            config,
            size,

            last_frame_instant: Instant::now(),
            render_pipelines: Vec::new(),
            fps: 0.0,
            delta_time: 1.0,
        }
    }

    /// opens the window and starts the [AppCreator::update] | [AppCreator::render] loop
    pub fn run(self) {
        let app = App {
            app_data: self.create_app_data(),

            state: self.state,

            window_event_fn: self.window_event_fn,
            resize_fn: self.resize_fn,
            update_fn: self.update_fn,
            render_fn: self.render_fn,
            init_fn: self.init_fn,
        };

        app.run(self.window, self.event_loop);
    }
}

pub type WindowEventFn<T> = fn(app_data: &AppData, state: &mut T, window_event: &WindowEvent);

pub type ResizeFn<T> = fn(app_data: &AppData, state: &mut T, size: &PhysicalSize<u32>);

pub type UpdateFn<T> = fn(app_data: &AppData, state: &mut T);

pub type RenderFn<T> = fn(app_data: &AppData, state: &mut T, command_encoder: CommandEncoder, texture_view: TextureView);

pub type InitFn<T> = fn(app_data: &AppData, state: &mut T, render_pipelines: &mut Vec<RenderPipeline>);