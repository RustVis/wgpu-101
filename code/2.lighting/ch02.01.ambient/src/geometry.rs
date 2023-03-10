// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Geometry data for common 3d shapes.

#![allow(dead_code)]

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
    //v_index += 1;

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

#[inline]
#[must_use]
pub fn create_cylinder() -> GeometryData {
    create_cylinder_detail(1.0, 2.0, 20, 10, 1.0, 1.0)
}

pub fn create_cylinder_detail(
    radius: f32,
    height: f32,
    slices: u32,
    stacks: u32,
    tex_u: f32,
    tex_v: f32,
) -> GeometryData {
    let mut geo_data = GeometryData::default();

    let vertex_count: usize = ((slices + 1) * (stacks + 3) + 2) as usize;
    let index_count: usize = (6 * slices * (stacks + 1)) as usize;

    let slices_f32 = slices as f32;
    let stacks_f32 = stacks as f32;

    geo_data.vertices.resize(vertex_count, [0.0, 0.0, 0.0]);
    geo_data.tex_coords.resize(vertex_count, [0.0, 0.0]);

    if index_count > INDICES32_THRESHOLD {
        geo_data.indices32.resize(index_count, 0);
    } else {
        geo_data.indices16.resize(index_count, 0);
    }

    let h2 = height / 2.0;
    let mut theta;
    let per_theta = 2.0 * PI / slices_f32;
    let stack_height = height / stacks_f32;

    // Out surface
    {
        let mut v_index: usize = 0;
        for i in 0..=stacks {
            let y: f32 = -h2 + i as f32 * stack_height;
            for j in 0..=slices {
                theta = j as f32 * per_theta;
                let u = theta / 2.0 / PI;
                let v = 1.0 - i as f32 / stacks_f32;

                geo_data.vertices[v_index] = [radius * theta.cos(), y, radius * theta.sin()];
                geo_data.tex_coords[v_index] = [u * tex_u, v * tex_v];
                v_index += 1;
            }
        }

        let mut i_index: usize = 0;
        for i in 0..stacks {
            for j in 0..slices {
                if index_count > INDICES32_THRESHOLD {
                    geo_data.indices32[i_index] = i * (slices + 1) + j;
                    i_index += 1;
                    geo_data.indices32[i_index] = (i + 1) * (slices + 1) + j;
                    i_index += 1;
                    geo_data.indices32[i_index] = (i + 1) * (slices + 1) + j + 1;
                    i_index += 1;

                    geo_data.indices32[i_index] = i * (slices + 1) + j;
                    i_index += 1;
                    geo_data.indices32[i_index] = (i + 1) * (slices + 1) + j + 1;
                    i_index += 1;
                    geo_data.indices32[i_index] = i * (slices + 1) + j + 1;
                    i_index += 1;
                } else {
                    geo_data.indices16[i_index] = (i * (slices + 1) + j) as u16;
                    i_index += 1;
                    geo_data.indices16[i_index] = ((i + 1) * (slices + 1) + j) as u16;
                    i_index += 1;
                    geo_data.indices16[i_index] = ((i + 1) * (slices + 1) + j + 1) as u16;
                    i_index += 1;

                    geo_data.indices16[i_index] = (i * (slices + 1) + j) as u16;
                    i_index += 1;
                    geo_data.indices16[i_index] = ((i + 1) * (slices + 1) + j + 1) as u16;
                    i_index += 1;
                    geo_data.indices16[i_index] = (i * (slices + 1) + j + 1) as u16;
                    i_index += 1;
                }
            }
        }
    }

    // Top and bottom
    {
        let mut v_index = ((slices + 1) * (stacks + 1)) as usize;
        let mut i_index = (6 * slices * stacks) as usize;
        let mut offset = v_index as u32;

        // Center point of top circular
        geo_data.vertices[v_index] = [0.0, h2, 0.0];
        geo_data.tex_coords[v_index] = [0.5, 0.5];
        v_index += 1;

        // Top circular
        for i in 0..=slices {
            theta = i as f32 * per_theta;
            let u = theta.cos() * radius / height + 0.5;
            let v = theta.sin() * radius / height + 0.5;
            geo_data.vertices[v_index] = [radius * theta.cos(), h2, radius * theta.sin()];
            geo_data.tex_coords[v_index] = [u, v];
            v_index += 1;
        }

        // Center point of bottom circular
        geo_data.vertices[v_index] = [0.0, -h2, 0.0];
        geo_data.tex_coords[v_index] = [0.5, 0.5];
        v_index += 1;

        // Bottom circular
        for i in 0..=slices {
            theta = i as f32 * per_theta;
            let u = theta.cos() * radius / height + 0.5;
            let v = theta.sin() * radius / height + 0.5;
            geo_data.vertices[v_index] = [radius * theta.cos(), -h2, radius * theta.sin()];
            geo_data.tex_coords[v_index] = [u, v];
            v_index += 1;
        }

        // Indices of top circular
        for i in 1..=slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = offset;
                i_index += 1;
                geo_data.indices32[i_index] = offset + i % (slices + 1) + 1;
                i_index += 1;
                geo_data.indices32[i_index] = offset + i;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = offset as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + i % (slices + 1) + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + i) as u16;
                i_index += 1;
            }
        }

        // Indices of bottom circular
        offset += slices + 2;
        for i in 1..=slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = offset;
                i_index += 1;
                geo_data.indices32[i_index] = offset + i;
                i_index += 1;
                geo_data.indices32[i_index] = offset + i % (slices + 1) + 1;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = offset as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + i) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + i % (slices + 1) + 1) as u16;
                i_index += 1;
            }
        }
    }

    geo_data
}

#[inline]
#[must_use]
pub fn create_cone() -> GeometryData {
    create_cone_detail(1.0, 2.0, 20)
}

pub fn create_cone_detail(radius: f32, height: f32, slices: u32) -> GeometryData {
    let mut geo_data = GeometryData::default();

    let vertex_count = (3 * slices + 1) as usize;
    let index_count = (6 * slices) as usize;

    geo_data.vertices.resize(vertex_count, [0.0, 0.0, 0.0]);
    geo_data.tex_coords.resize(vertex_count, [0.0, 0.0]);

    if index_count > 65535 {
        geo_data.indices32.resize(index_count, 0);
    } else {
        geo_data.indices16.resize(index_count, 0);
    }

    let h2 = height / 2.0;
    let mut theta;
    let per_theta = 2.0 * PI / slices as f32;

    // Side face
    {
        let mut i_index: usize = 0;
        let mut v_index: usize = 0;

        for _i in 0..slices {
            geo_data.vertices[v_index] = [0.0, h2, 0.0];
            geo_data.tex_coords[v_index] = [0.5, 0.5];
            v_index += 1;
        }

        for i in 0..slices {
            theta = i as f32 * per_theta;
            geo_data.vertices[v_index] = [radius * theta.cos(), -h2, radius * theta.sin()];
            geo_data.tex_coords[v_index] = [theta.cos() / 2.0 + 0.5, theta.sin() / 2.0 + 0.5];
            v_index += 1;
        }

        // Indices
        for i in 0..slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = i;
                i_index += 1;
                geo_data.indices32[i_index] = slices + (i + 1) % slices;
                i_index += 1;
                geo_data.indices32[i_index] = slices + i % slices;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = i as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (slices + (i + 1) % slices) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (slices + i % slices) as u16;
                i_index += 1;
            }
        }
    }

    // Bottom side
    {
        let mut i_index = (3 * slices) as usize;
        let mut v_index = (2 * slices) as usize;

        // Top point
        for i in 0..slices {
            theta = i as f32 * per_theta;

            geo_data.vertices[v_index] = [radius * theta.cos(), -h2, radius * theta.sin()];
            geo_data.tex_coords[v_index] = [theta.cos() / 2.0 + 0.5, theta.sin() / 2.0 + 0.5];
            v_index += 1;
        }

        // Center point of bottom circular.
        geo_data.vertices[v_index] = [0.0, -h2, 0.0];
        geo_data.tex_coords[v_index] = [0.5, 0.5];
        //v_index += 1;

        // Indices
        let offset = 2 * slices;
        for i in 0..slices {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = offset + slices;
                i_index += 1;
                geo_data.indices32[i_index] = offset + i % slices;
                i_index += 1;
                geo_data.indices32[i_index] = offset + (i + 1) % slices;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = (offset + slices) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + i % slices) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (offset + (i + 1) % slices) as u16;
                i_index += 1;
            }
        }
    }

    geo_data
}

#[inline]
#[must_use]
pub fn create_plane() -> GeometryData {
    create_plane_detail(10.0, 10.0, 1.0, 1.0)
}

pub fn create_plane_detail(width: f32, depth: f32, tex_u: f32, tex_v: f32) -> GeometryData {
    let mut geo_data = GeometryData::default();

    geo_data.vertices.resize(4, [0.0, 0.0, 0.0]);
    geo_data.tex_coords.resize(4, [0.0, 0.0]);

    let w2 = width / 2.0;
    let d2 = depth / 2.0;

    let mut v_index: usize = 0;
    geo_data.vertices[v_index] = [-w2, 0.0, -d2];
    geo_data.tex_coords[v_index] = [0.0, tex_v];
    v_index += 1;

    geo_data.vertices[v_index] = [-w2, 0.0, d2];
    geo_data.tex_coords[v_index] = [0.0, 0.0];
    v_index += 1;

    geo_data.vertices[v_index] = [w2, 0.0, d2];
    geo_data.tex_coords[v_index] = [tex_u, 0.0];
    v_index += 1;

    geo_data.vertices[v_index] = [w2, 0.0, -d2];
    geo_data.tex_coords[v_index] = [tex_u, tex_v];
    //v_index += 1;

    geo_data.indices16 = vec![0, 1, 2, 2, 3, 0];

    geo_data
}

#[inline]
#[must_use]
pub fn create_grid() -> GeometryData {
    create_grid_detail(20.0, 20.0, 20, 20, 1.0, 1.0)
}

pub fn create_grid_detail(
    grid_width: f32,
    grid_depth: f32,
    slices_x: u32,
    slices_y: u32,
    tex_u: f32,
    tex_v: f32,
) -> GeometryData {
    let mut geo_data = GeometryData::default();

    let vertex_count = ((slices_x + 1) * (slices_y + 1)) as usize;
    let index_count = (6 * slices_x * slices_y) as usize;

    geo_data.vertices.resize(vertex_count, [0.0, 0.0, 0.0]);
    geo_data.tex_coords.resize(vertex_count, [0.0, 0.0]);

    if index_count > INDICES32_THRESHOLD {
        geo_data.indices32.resize(index_count, 0);
    } else {
        geo_data.indices16.resize(index_count, 0);
    }

    let mut v_index: usize = 0;
    let mut i_index: usize = 0;

    let slice_width = grid_width / slices_x as f32;
    let slice_depth = grid_depth / slices_y as f32;
    let left_bottom_x = -grid_width / 2.0;
    let left_bottom_z = -grid_depth / 2.0;
    let mut pos_x;
    let mut pos_z;
    let slice_tex_width = tex_u / slices_x as f32;
    let slice_tex_depth = tex_v / slices_y as f32;

    // Vertices
    //  __ __
    // | /| /|
    // |/_|/_|
    // | /| /|
    // |/_|/_|
    for z in 0..=slices_y {
        let z = z as f32;
        pos_z = left_bottom_z + z * slice_depth;

        for x in 0..=slices_x {
            let x = x as f32;
            pos_x = left_bottom_x + x * slice_width;

            geo_data.vertices[v_index] = [pos_x, 0.0, pos_z];
            geo_data.tex_coords[v_index] = [x * slice_tex_width, tex_v - z * slice_tex_depth];
            v_index += 1;
        }
    }

    // Indices
    for i in 0..slices_y {
        for j in 0..slices_x {
            if index_count > INDICES32_THRESHOLD {
                geo_data.indices32[i_index] = i * (slices_x + 1) + j;
                i_index += 1;
                geo_data.indices32[i_index] = (i + 1) * (slices_x + 1) + j;
                i_index += 1;
                geo_data.indices32[i_index] = (i + 1) * (slices_x + 1) + j + 1;
                i_index += 1;

                geo_data.indices32[i_index] = (i + 1) * (slices_x + 1) + j + 1;
                i_index += 1;
                geo_data.indices32[i_index] = i * (slices_x + 1) + j + 1;
                i_index += 1;
                geo_data.indices32[i_index] = i * (slices_x + 1) + j;
                i_index += 1;
            } else {
                geo_data.indices16[i_index] = (i * (slices_x + 1) + j) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = ((i + 1) * (slices_x + 1) + j) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = ((i + 1) * (slices_x + 1) + j + 1) as u16;
                i_index += 1;

                geo_data.indices16[i_index] = ((i + 1) * (slices_x + 1) + j + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (i * (slices_x + 1) + j + 1) as u16;
                i_index += 1;
                geo_data.indices16[i_index] = (i * (slices_x + 1) + j) as u16;
                i_index += 1;
            }
        }
    }

    geo_data
}
