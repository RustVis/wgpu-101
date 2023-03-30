// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 3],
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pad: [f32; 4],
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: [-1.5, 1.5, 2.0],
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.5, 0.5, 0.5],
            specular: [1.0, 1.0, 1.0],
            pad: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

pub type LightBytes = [f32; mem::size_of::<Light>() / mem::size_of::<f32>()];
pub type LightRef<'a> = &'a LightBytes;

impl AsRef<LightBytes> for Light {
    fn as_ref(&self) -> LightRef {
        unsafe { mem::transmute(self) }
    }
}
