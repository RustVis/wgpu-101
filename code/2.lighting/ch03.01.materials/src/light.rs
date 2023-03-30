// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::Vector4;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Vector4<f32>,
    pub ambient: Vector4<f32>,
    pub diffuse: Vector4<f32>,
    pub specular: Vector4<f32>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: Vector4::new(-1.5, 1.5, 2.0, 1.0),
            ambient: Vector4::new(0.2, 0.2, 0.2, 1.0),
            diffuse: Vector4::new(0.5, 0.5, 0.5, 1.0),
            specular: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

pub type LightBytes = [f32; 16];
pub type LightRef<'a> = &'a LightBytes;

impl AsRef<LightBytes> for Light {
    fn as_ref(&self) -> LightRef {
        unsafe { mem::transmute(self) }
    }
}
