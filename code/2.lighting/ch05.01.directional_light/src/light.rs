// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::Vector3;
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub direction: Vector3<f32>,
    pad0: f32,

    pub ambient: Vector3<f32>,
    pad1: f32,
    pub diffuse: Vector3<f32>,
    pad2: f32,
    pub specular: Vector3<f32>,
    pad3: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            direction: Vector3::new(-0.2, -1.0, -0.3),
            pad0: 1.0,

            ambient: Vector3::new(0.2, 0.2, 0.2),
            pad1: 1.0,
            diffuse: Vector3::new(0.5, 0.5, 0.5),
            pad2: 1.0,
            specular: Vector3::new(1.0, 1.0, 1.0),
            pad3: 1.0,
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
