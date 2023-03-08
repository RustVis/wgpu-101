// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::{perspective, Deg, Matrix4, One, Point3, Vector3};
use std::mem;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    target: Point3<f32>,
    eye: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    zoom_near: f32,
    zoom_far: f32,

    uniform: CameraUniform,
}

impl Camera {
    pub fn new(eye: Point3<f32>, aspect: f32) -> Self {
        let mut instance = Self {
            // Default target is the origin point.
            target: (0.0, 0.0, 0.0).into(),
            eye,
            up: Vector3::unit_y(),
            aspect,
            fovy: 45.0,
            zoom_near: 0.1,
            zoom_far: 100.0,

            uniform: CameraUniform::default(),
        };
        instance.update_uniform();
        instance
    }

    fn update_uniform(&mut self) {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.zoom_near, self.zoom_far);
        self.uniform.view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }

    pub fn uniform_ref(&self) -> CameraUniformRef {
        self.uniform.as_ref()
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CameraUniform {
    pub view_proj: Matrix4<f32>,
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Matrix4::one(),
        }
    }
}

pub type CameraUniformBytes = [f32; 16];
pub type CameraUniformRef<'a> = &'a CameraUniformBytes;

impl AsRef<CameraUniformBytes> for CameraUniform {
    fn as_ref(&self) -> CameraUniformRef {
        unsafe { mem::transmute(self) }
    }
}
