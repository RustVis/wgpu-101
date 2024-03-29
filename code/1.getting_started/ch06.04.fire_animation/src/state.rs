// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use instant::Instant;
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::texture::Texture;
use crate::vertex::{Vertex, INDICES, VERTICES};
use crate::Error;

#[cfg(not(target_arch = "wasm32"))]
const ANIMATION_FRAMES: u32 = 120;
#[cfg(target_arch = "wasm32")]
const ANIMATION_FRAMES: u32 = 16;
const ANIMATION_SPEED: f32 = 1000.0 / 16.0;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    pub index: u32,
}

#[derive(Debug)]
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Window,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    texture_bind_group: wgpu::BindGroup,

    start_time: Instant,
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

        let mut limits = if cfg!(target_arch = "wasm32") {
            wgpu::Limits::downlevel_webgl2_defaults()
        } else {
            wgpu::Limits::default()
        };
        limits.max_sampled_textures_per_shader_stage = ANIMATION_FRAMES;
        limits.max_samplers_per_shader_stage = ANIMATION_FRAMES;
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::TEXTURE_BINDING_ARRAY
                        | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY,
                    limits,
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
            .find(wgpu::TextureFormat::is_srgb)
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

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../res/shaders/texture.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        render_pipeline
    }

    fn create_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        (vertex_buffer, index_buffer)
    }

    fn load_texture_files(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Vec<Texture>, Error> {
        let mut textures = Vec::new();
        for index in 1..=ANIMATION_FRAMES {
            let name = format!("fire{index:0>3}.png");
            let filepath = format!("./res/textures/fire/fire{index:0>3}.png");
            let texture = Texture::from_file(device, queue, &filepath, Some(&name))?;
            textures.push(texture);
        }

        Ok(textures)
    }

    fn create_texture_bind_group(
        device: &wgpu::Device,
        uniforms: Uniforms,
        textures: &[Texture],
    ) -> Result<(wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup), Error> {
        let uniforms_ref = &[uniforms];
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(uniforms_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let texture_count = textures.len() as u32;

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: NonZeroU32::new(texture_count),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(texture_count),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let view_list = textures
            .iter()
            .map(|texture| &texture.view)
            .collect::<Vec<_>>();
        let sampler_list = textures
            .iter()
            .map(|texture| &texture.sampler)
            .collect::<Vec<_>>();
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&view_list),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(&sampler_list),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("texture_bind_group"),
        });

        Ok((
            uniform_buffer,
            texture_bind_group_layout,
            texture_bind_group,
        ))
    }

    pub async fn new(window: Window) -> Result<Self, Error> {
        let (surface, device, queue, config, size) = Self::create_surface(&window).await?;

        let fire_textures = Self::load_texture_files(&device, &queue)?;
        let uniforms = Uniforms { index: 0 };
        let (uniform_buffer, texture_bind_group_layout, texture_bind_group) =
            Self::create_texture_bind_group(&device, uniforms, &fire_textures)?;

        let bind_group_layouts = [&texture_bind_group_layout];
        let render_pipeline = Self::create_render_pipeline(&device, &config, &bind_group_layouts);

        let (vertex_buffer, index_buffer) = Self::create_buffers(&device);
        let num_indices = INDICES.len() as u32;

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,

            render_pipeline,

            vertex_buffer,
            index_buffer,
            num_indices,

            uniforms,
            uniform_buffer,
            texture_bind_group,

            start_time: Instant::now(),
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
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        let dt = self.start_time.elapsed();
        let dt = ANIMATION_SPEED * dt.as_secs_f32();
        let dt = dt as u32;
        self.uniforms.index = dt % ANIMATION_FRAMES;
        let uniforms_ref = &[self.uniforms];

        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(uniforms_ref));
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
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
