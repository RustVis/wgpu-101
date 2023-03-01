// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use instant::Instant;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::frames::ColorWindow;
use crate::uniforms::{Uniforms, UniformsRef};
use crate::vertex::{Vertex, VERTICES};
use crate::Error;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Window,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    render_pipeline: wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,

    start_time: Instant,
    fps_time: Instant,
    frames: u32,

    egui_platform: Platform,
    egui_render_pass: RenderPass,
    ui_window: ColorWindow,
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
            wgpu::TextureFormat,
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
                    features: wgpu::Features::empty(),
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
            // Immediate mode is not supported on linux wayland desktop.
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok((surface, device, queue, config, size, surface_format))
    }

    fn create_uniform_buffer(
        device: &wgpu::Device,
        uniforms_ref: UniformsRef,
    ) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(uniforms_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        (
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        )
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../res/shaders/triangle.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[uniform_bind_group_layout],
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

    fn create_egui_platform(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        size: PhysicalSize<u32>,
        scale_factor: f64,
    ) -> (Platform, RenderPass, ColorWindow) {
        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor,
            ..Default::default()
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let render_pass = RenderPass::new(device, surface_format, 1);

        // Display the demo application that ships with egui.
        let ui_window = ColorWindow::default();

        (platform, render_pass, ui_window)
    }

    pub async fn new(window: Window) -> Result<Self, Error> {
        let (surface, device, queue, config, size, surface_format) =
            Self::create_surface(&window).await?;

        let (egui_platform, egui_render_pass, ui_window) =
            Self::create_egui_platform(&device, surface_format, size, window.scale_factor());

        let uniforms = Uniforms::from_color(*ui_window.color());
        let uniforms_ref = uniforms.as_ref();

        let (uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            Self::create_uniform_buffer(&device, uniforms_ref);

        let render_pipeline =
            Self::create_render_pipeline(&device, &config, &uniform_bind_group_layout);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,

            uniforms,
            uniform_buffer,
            uniform_bind_group,

            render_pipeline,

            vertex_buffer,
            num_vertices,

            start_time: Instant::now(),
            fps_time: Instant::now(),
            frames: 0,

            egui_platform,
            egui_render_pass,
            ui_window,
        })
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn platform_mut(&mut self) -> &mut Platform {
        &mut self.egui_platform
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
        let dt = self.fps_time.elapsed().as_secs_f64();
        let fps: u32 = (f64::from(self.frames) / dt).round() as u32;
        log::info!("fps: {fps}");
        if dt > 1.0 {
            self.frames = 0;
            self.fps_time = Instant::now();
        }
        self.frames += 1;

        let dt = self.start_time.elapsed().as_secs_f64();
        self.egui_platform.update_time(dt);

        self.uniforms.color = *self.ui_window.color();
        let uniforms_ref = self.uniforms.as_ref();
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

        // Draw triangle.
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        // Draw the egui UI frame.
        {
            self.egui_platform.begin_frame();
            self.ui_window.ui(&self.egui_platform.context());
            let full_output = self.egui_platform.end_frame(Some(&self.window));
            let paint_jobs = self.egui_platform.context().tessellate(full_output.shapes);
            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: self.config.width,
                physical_height: self.config.height,
                scale_factor: self.window.scale_factor() as f32,
            };
            let tdelta: egui::TexturesDelta = full_output.textures_delta;
            self.egui_render_pass
                .add_textures(&self.device, &self.queue, &tdelta)
                .expect("add texture ok");
            self.egui_render_pass.update_buffers(
                &self.device,
                &self.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            // Record all render passes.
            self.egui_render_pass
                .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
                .unwrap();
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
