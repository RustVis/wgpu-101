// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Geometry data for common 3d shapes.

use std::iter::zip;

use crate::vertex::Vertex;

#[derive(Debug, Default, Clone)]
pub struct GeometryData {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
    pub tex_coords: Vec<[f32; 2]>,
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

    let indices = vec![
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
        indices,
    }
}
