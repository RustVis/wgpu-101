// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::Vector4;
use std::mem;
use wgpu::util::DeviceExt;

use crate::light::Light;
use crate::scenes::create_vertex;
use crate::texture::Texture;
use crate::vertex::Vertex;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub ambient: Vector4<f32>,
    pub diffuse: Vector4<f32>,
    pub specular: Vector4<f32>,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vector4::new(1.0, 0.5, 0.31, 1.0),
            diffuse: Vector4::new(1.0, 0.5, 0.31, 1.0),
            specular: Vector4::new(0.5, 0.5, 0.5, 1.0),
            shininess: 32.0,
        }
    }
}

pub type MaterialBytes = [f32; 16];
pub type MaterialRef<'a> = &'a MaterialBytes;

impl AsRef<MaterialBytes> for Material {
    fn as_ref(&self) -> MaterialRef {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Debug)]
pub struct BoxScene {
    pub render_pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,

    pub material: Material,
    pub light: Light,
    pub material_buffer: wgpu::Buffer,
    pub light_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

impl BoxScene {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let (
            material,
            light,
            material_buffer,
            light_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        ) = Self::create_uniform(device);

        let bind_group_layouts = [camera_bind_group_layout, &uniform_bind_group_layout];
        let render_pipeline = Self::create_render_pipeline(device, config, &bind_group_layouts);

        let (vertex_buffer, index_buffer, num_indices) = create_vertex(device);

        Self {
            render_pipeline,

            vertex_buffer,
            index_buffer,
            num_indices,

            material,
            light,
            material_buffer,
            light_buffer,
            uniform_bind_group,
        }
    }

    pub fn create_uniform(
        device: &wgpu::Device,
    ) -> (
        Material,
        Light,
        wgpu::Buffer,
        wgpu::Buffer,
        wgpu::BindGroupLayout,
        wgpu::BindGroup,
    ) {
        let material = Material::default();
        let light = Light::default();

        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Box Material Buffer"),
            contents: bytemuck::cast_slice(material.as_ref()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Box Light Buffer"),
            contents: bytemuck::cast_slice(light.as_ref()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Box Buffer Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: light_buffer.as_entire_binding(),
                },
            ],
            label: Some("Box Bind Group"),
        });

        (
            material,
            light,
            material_buffer,
            light_buffer,
            bind_group_layout,
            bind_group,
        )
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Box Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../res/shaders/box.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Box Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Box Render Pipeline"),
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
