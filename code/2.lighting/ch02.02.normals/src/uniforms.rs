// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::{Matrix4, One};
use std::mem;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms {
    pub model: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            model: Matrix4::one(),
            view: Matrix4::one(),
            projection: Matrix4::one(),
        }
    }
}

pub type UniformsBytes = [f32; 48];
pub type UniformsRef<'a> = &'a UniformsBytes;

impl AsRef<UniformsBytes> for Uniforms {
    fn as_ref(&self) -> UniformsRef {
        unsafe { mem::transmute(self) }
    }
}
