// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Geometry data for common 3d shapes.

use std::f32::consts::PI;
use std::iter::zip;

use crate::vertex::Vertex;

pub const INDICES32_THRESHOLD: usize = u16::MAX as usize;

#[derive(Debug, Default, Clone)]
pub struct GeometryData {
    pub vertices: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub indices16: Vec<u16>,
    pub indices32: Vec<u32>,
}

impl GeometryData {
    pub fn vertex_data(&self) -> Vec<Vertex> {
        let mut list = Vec::with_capacity(self.vertices.len());
        for (vertex, tex_coord) in zip(self.vertices.iter(), self.tex_coords.iter()) {
            list.push(Vertex(*vertex, *tex_coord));
        }
        list
    }
}

#[must_use]
pub fn create_cube() -> GeometryData {
    create_cube_detail(2.0, 2.0, 2.0)
}

#[must_use]
pub fn create_cube_detail(width: f32, height: f32, depth: f32) -> GeometryData {
    let w2 = width / 2.0;
    let h2 = height / 2.0;
    let d2 = depth / 2.0;

    let vertices = vec![
        // right(+X)
        [w2, -h2, -d2],
        [w2, h2, -d2],
        [w2, h2, d2],
        [w2, -h2, d2],
        // left(-X)
        [-w2, -h2, d2],
        [-w2, h2, d2],
        [-w2, h2, -d2],
        [-w2, -h2, -d2],
        // top(+Y)
        [-w2, h2, -d2],
        [-w2, h2, d2],
        [w2, h2, d2],
        [w2, h2, -d2],
        // bottom(-Y)
        [w2, -h2, -d2],
        [w2, -h2, d2],
        [-w2, -h2, d2],
        [-w2, -h2, -d2],
        // rear(+Z)
        [w2, -h2, d2],
        [w2, h2, d2],
        [-w2, h2, d2],
        [-w2, -h2, d2],
        // front(-Z)
        [-w2, -h2, -d2],
        [-w2, h2, -d2],
        [w2, h2, -d2],
        [w2, -h2, -d2],
    ];

    let mut tex_coords = vec![[0.0, 0.0]; 24];
    for i in 0..6 {
        tex_coords[i * 4] = [0.0, 1.0];
        tex_coords[i * 4 + 1] = [0.0, 0.0];
        tex_coords[i * 4 + 2] = [1.0, 0.0];
        tex_coords[i * 4 + 3] = [1.0, 1.0];
    }

    let indices16 = vec![
        0, 1, 2, 2, 3, 0, // right(+X)
        4, 5, 6, 6, 7, 4, // left(-X)
        8, 9, 10, 10, 11, 8, // top(+Y)
        12, 13, 14, 14, 15, 12, // bottom(-Y)
        16, 17, 18, 18, 19, 16, // rear(+Z)
        20, 21, 22, 22, 23, 20, // front(-Z)
    ];

    GeometryData {
        vertices,
        tex_coords,
        indices16,
        indices32: Vec::new(),
    }
}

#[inline]
#[must_use]
pub fn create_sphere() -> GeometryData {
    create_sphere_detail(1.0, 20, 20)
}

#[must_use]
pub fn create_sphere_detail(radius: f32, levels: u32, slices: u32) -> GeometryData {
    let vertex_count: usize = (2 + (levels - 1) * (slices + 1)) as usize;
    let index_count: usize = (6 * (levels - 1) * slices) as usize;
    let mut geo_data = GeometryData::default();
    geo_data.vertices.resize(vertex_count, [0.0, 0.0, 0.0]);
    geo_data.tex_coords.resize(vertex_count, [0.0, 0.0]);
    if index_count > INDICES32_THRESHOLD {
        geo_data.indices32.resize(index_count, 0);
    } else {
        geo_data.indices16.resize(index_count, 0);
    }

    let mut v_index: usize = 0;
    let mut i_index: usize = 0;

    let mut phi: f32;
    let mut theta: f32;
    let per_phi = PI / levels as f32;
    let per_theta = 2.0 * PI / slices as f32;
    let mut x: f32;
    let mut y: f32;
    let mut z: f32;

    // Top vertex
    geo_data.vertices[v_index] = [0.0, radius, 0.0];
    geo_data.tex_coords[v_index] = [0.0, 0.0];
    v_index += 1;

    for i in 1..levels {
        phi = per_phi * i as f32;
        for j in 0..=slices {
            theta = per_theta * j as f32;
            x = radius * phi.sin() * theta.cos();
            y = radius * phi.cos();
            z = radius * phi.sin() * theta.sin();
            let pos = [x, y, z];

            geo_data.vertices[v_index] = pos;
            geo_data.tex_coords[v_index] = [theta / 2.0 / PI, phi / PI];
            v_index += 1;
        }
    }

    // Bottom vertex
    geo_data.vertices[v_index] = [0.0, -radius, 0.0];
    geo_data.tex_coords[v_index] = [0.0, 1.0];
    v_index += 1;

    if levels > 1 {
        for j in 1..=slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = 0;
                i_index += 1;
                geo_data.indices32[i_index] = j % (slices + 1) + 1;
                i_index += 1;
                geo_data.indices32[i_index] = j;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = 0;
                i_index += 1;
                geo_data.indices16[i_index] = (j % (slices + 1) + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = j as u16;
                i_index += 1;
            }
        }
    }

    for i in 1..(levels - 1) {
        for j in 1..=slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = (i - 1) * (slices + 1) + j;
                i_index += 1;
                geo_data.indices32[i_index] = (i - 1) * (slices + 1) + j % (slices + 1) + 1;
                i_index += 1;
                geo_data.indices32[i_index] = i * (slices + 1) + j % (slices + 1) + 1;
                i_index += 1;

                geo_data.indices32[i_index] = i * (slices + 1) + j % (slices + 1) + 1;
                i_index += 1;
                geo_data.indices32[i_index] = i * (slices + 1) + j;
                i_index += 1;
                geo_data.indices32[i_index] = (i - 1) * (slices + 1) + j;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = ((i - 1) * (slices + 1) + j) as u16;
                i_index += 1;
                geo_data.indices16[i_index] =
                    ((i - 1) * (slices + 1) + j % (slices + 1) + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (i * (slices + 1) + j % (slices + 1) + 1) as u16;
                i_index += 1;

                geo_data.indices16[i_index] = (i * (slices + 1) + j % (slices + 1) + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (i * (slices + 1) + j) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = ((i - 1) * (slices + 1) + j) as u16;
                i_index += 1;
            }
        }
    }

    if levels > 1 {
        for j in 1..=slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = (levels - 2) * (slices + 1) + j;
                i_index += 1;
                geo_data.indices32[i_index] = (levels - 2) * (slices + 1) + j % (slices + 1) + 1;
                i_index += 1;
                geo_data.indices32[i_index] = (levels - 1) * (slices + 1) + 1;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = ((levels - 2) * (slices + 1) + j) as u16;
                i_index += 1;
                geo_data.indices16[i_index] =
                    ((levels - 2) * (slices + 1) + j % (slices + 1) + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = ((levels - 1) * (slices + 1) + 1) as u16;
                i_index += 1;
            }
        }
    }

    geo_data
}
