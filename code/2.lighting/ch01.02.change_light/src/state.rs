// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::box_scene::BoxScene;
use crate::camera::Camera;
use crate::light_scene::LightScene;
use crate::texture::Texture;
use crate::Error;

#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Window,

    box_scene: BoxScene,
    light_scene: LightScene,

    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    depth_texture: Texture,
}

impl State {
    async fn create_surface(
        window: &Window,
    ) -> Result<
        (
            wgpu::Surface,
            wgpu::Device,
            wgpu::Queue,
            wgpu::SurfaceConfiguration,
            PhysicalSize<u32>,
        ),
        Error,
    > {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| {
                Error::Others("Failed to get an approprivate wgpu adapter".to_owned())
            })?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: if cfg!(target_arch = "wasm32") {
                        wgpu::Features::empty()
                    } else {
                        wgpu::Features::POLYGON_MODE_LINE
                    },
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok((surface, device, queue, config, size))
    }

    fn create_camera(
        device: &wgpu::Device,
        size: PhysicalSize<u32>,
    ) -> Result<(Camera, wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup), Error> {
        let eye_pos = (0.0, 1.0, 2.0).into();
        let aspect = size.width as f32 / size.height as f32;
        let camera = Camera::new(eye_pos, aspect);

        let uniform_ref = camera.uniform_ref();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Ok((
            camera,
            camera_buffer,
            camera_bind_group_layout,
            camera_bind_group,
        ))
    }

    pub async fn new(window: Window) -> Result<Self, Error> {
        let (surface, device, queue, config, size) = Self::create_surface(&window).await?;

        let (camera, camera_buffer, camera_bind_group_layout, camera_bind_group) =
            Self::create_camera(&device, size)?;

        let box_scene = BoxScene::new(&device, &config, &camera_bind_group_layout);
        let light_scene = LightScene::new(&device, &config, &camera_bind_group_layout);

        let depth_texture = Texture::create_depth_texture(&device, size, Some("Depth Texture"));

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,

            box_scene,
            light_scene,

            camera,
            camera_buffer,
            camera_bind_group,

            depth_texture,
        })
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture =
                Texture::create_depth_texture(&self.device, self.size, Some("Depth Texture"));
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera.process_event(event)
    }

    pub fn update(&mut self) {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(self.camera.uniform_ref()),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.3,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.box_scene.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.box_scene.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.box_scene.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.box_scene.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.box_scene.num_indices, 0, 0..1);

            render_pass.set_pipeline(&self.light_scene.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.light_scene.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.light_scene.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.light_scene.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.light_scene.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
