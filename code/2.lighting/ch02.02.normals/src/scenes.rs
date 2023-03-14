// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use wgpu::util::DeviceExt;

use crate::geometry::create_cube;

pub fn create_vertex(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let geometry_data = create_cube();
    let vertices = geometry_data.vertex_data();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&geometry_data.indices16),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = geometry_data.indices16.len() as u32;

    (vertex_buffer, index_buffer, num_indices)
}
