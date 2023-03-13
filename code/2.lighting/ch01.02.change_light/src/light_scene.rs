// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::{Matrix4, One, Vector3};
use std::mem;
use wgpu::util::DeviceExt;

use crate::scenes::create_vertex;
use crate::texture::Texture;
use crate::transforms;
use crate::vertex::Vertex;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct LightUniform {
    pub model: Matrix4<f32>,
}

impl Default for LightUniform {
    fn default() -> Self {
        Self {
            model: Matrix4::one(),
        }
    }
}

pub type LightUniformBytes = [f32; 16];
pub type LightUniformRef<'a> = &'a LightUniformBytes;

impl AsRef<LightUniformBytes> for LightUniform {
    fn as_ref(&self) -> LightUniformRef {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Debug)]
pub struct LightScene {
    pub render_pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,

    pub uniform: LightUniform,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl LightScene {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let (uniform, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            Self::create_uniform(device);

        let bind_group_layouts = [camera_bind_group_layout, &uniform_bind_group_layout];
        let render_pipeline = Self::create_render_pipeline(device, config, &bind_group_layouts);

        let (vertex_buffer, index_buffer, num_indices) = create_vertex(device);

        Self {
            render_pipeline,

            vertex_buffer,
            index_buffer,
            num_indices,

            uniform,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn create_uniform(
        device: &wgpu::Device,
    ) -> (
        LightUniform,
        wgpu::Buffer,
        wgpu::BindGroupLayout,
        wgpu::BindGroup,
    ) {
        let mut uniform = LightUniform::default();
        let light_pos = Vector3::new(1.0, 1.2, 2.0);
        transforms::translate(&mut uniform.model, light_pos);
        let light_scale = Vector3::new(0.2, 0.2, 0.2);
        transforms::scale(&mut uniform.model, light_scale);

        let uniform_ref = uniform.as_ref();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("Light Buffer Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Light Bind Group"),
        });

        (uniform, uniform_buffer, bind_group_layout, bind_group)
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Light Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../res/shaders/light.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Light Render Pipeline"),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        render_pipeline
    }
}
