use bytemuck::{Pod, Zeroable};
use nalgebra::Vector2;
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, Queue, RenderPipeline,
    RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceError, TextureViewDescriptor,
    VertexBufferLayout,
};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector2<f32>,
}
unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
impl Vertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

#[derive(Debug)]
pub struct Camera2d {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub size: Vector2<u32>,
    pub pipelines: HashMap<u64, RenderPipeline>,
}
impl Camera2d {
    pub async fn new<T: HasRawWindowHandle>(window: &T, size: Vector2<u32>) -> Self {
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    features: Features::empty(),
                },
                None,
            )
            .await
            .unwrap();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.x,
            height: size.y,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipelines: HashMap::new(),
        }
    }
    pub fn resize(&mut self, size: Vector2<u32>) {
        if size.x > 0 && size.y > 0 {
            self.size = size;
            self.config.width = size.x;
            self.config.height = size.y;
            self.surface.configure(&self.device, &self.config);
        }
    }
    pub fn update(&mut self) {
        todo!()
    }
    pub fn render(
        &mut self,
        //render_pipeline: RenderPipeline,
        //vertices: &[Vertex],
    ) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        /*let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });*/
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            //render_pass.set_pipeline(&render_pipeline);
            //render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            //render_pass.draw(0..vertices.len() as u32, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
