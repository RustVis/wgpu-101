// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::Vector3;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms {
    pub color: Vector3<f32>,
    pad: f32,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            color: Vector3::new(1.0, 1.0, 1.0),
            pad: 0.0,
        }
    }
}

impl Uniforms {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: Vector3::new(r, g, b),
            pad: 0.0,
        }
    }
    pub fn from_color(color: Vector3<f32>) -> Self {
        Self { color, pad: 0.0 }
    }
}

pub type UniformsBytes = [f32; 4];
pub type UniformsRef<'a> = &'a UniformsBytes;

impl AsRef<UniformsBytes> for Uniforms {
    fn as_ref(&self) -> UniformsRef {
        unsafe { mem::transmute(self) }
    }
}
