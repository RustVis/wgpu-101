// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex([f32; 3], [f32; 2]);

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub const fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // A
    Vertex([-0.5, -0.5, -0.5], [0.0, 0.0]),
    Vertex([0.5, -0.5, -0.5], [1.0, 0.0]),
    Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [0.0, 0.0]),
    // B
    Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex([0.5, 0.5, 0.5], [1.0, 1.0]),
    Vertex([0.5, 0.5, 0.5], [1.0, 1.0]),
    Vertex([-0.5, 0.5, 0.5], [0.0, 1.0]),
    Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
    // C
    Vertex([-0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex([-0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex([-0.5, 0.5, 0.5], [1.0, 0.0]),
    // D
    Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex([0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex([0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex([0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
    // E
    Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
    Vertex([0.5, -0.5, -0.5], [1.0, 1.0]),
    Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex([0.5, -0.5, 0.5], [1.0, 0.0]),
    Vertex([-0.5, -0.5, 0.5], [0.0, 0.0]),
    Vertex([-0.5, -0.5, -0.5], [0.0, 1.0]),
    // F
    Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
    Vertex([0.5, 0.5, -0.5], [1.0, 1.0]),
    Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex([0.5, 0.5, 0.5], [1.0, 0.0]),
    Vertex([-0.5, 0.5, 0.5], [0.0, 0.0]),
    Vertex([-0.5, 0.5, -0.5], [0.0, 1.0]),
];
