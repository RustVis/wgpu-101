// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::Vector3;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms {
    pub color0: Vector3<f32>,
    pub color1: Vector3<f32>,
    pub color2: Vector3<f32>,
    pad: Vector3<f32>,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            color0: Vector3::new(1.0, 1.0, 1.0),
            color1: Vector3::new(1.0, 1.0, 1.0),
            color2: Vector3::new(1.0, 1.0, 1.0),
            pad: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

pub type UniformsBytes = [f32; 3 * 4];
pub type UniformsRef<'a> = &'a UniformsBytes;

impl AsRef<UniformsBytes> for Uniforms {
    fn as_ref(&self) -> UniformsRef {
        unsafe { mem::transmute(self) }
    }
}
