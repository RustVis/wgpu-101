// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::mem;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms {
    pub vec3_val0: [f32; 3],
    pad: f32,
    pub vec3_val: [f32; 3],
    pub f32_val: f32,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            vec3_val0: [1.0, 0.5, 0.3],
            pad: 1.0,
            vec3_val: [1.0, 0.5, 0.3],
            f32_val: 1.0,
        }
    }
}

pub type UniformsBytes = [f32; 8];
pub type UniformsRef<'a> = &'a UniformsBytes;

impl AsRef<UniformsBytes> for Uniforms {
    fn as_ref(&self) -> UniformsRef {
        unsafe { mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms1 {
    pub vec4_val: [f32; 4],
    pub f32_val: f32,
    pub f32_val2: f32,
    pub f32_val3: f32,
    pub int_val: i32,
}

impl Default for Uniforms1 {
    fn default() -> Self {
        Self {
            vec4_val: [1.0, 0.5, 0.3, 1.0],
            f32_val: 1.0,
            f32_val2: 0.5,
            f32_val3: 0.3,
            int_val: 1,
        }
    }
}

pub type Uniforms1Bytes = [f32; 8];
pub type Uniforms1Ref<'a> = &'a Uniforms1Bytes;

impl AsRef<Uniforms1Bytes> for Uniforms1 {
    fn as_ref(&self) -> Uniforms1Ref {
        unsafe { mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Uniforms2 {
    pub vec4_val: [f32; 4],
    pub vec3_val0: [f32; 3],
    pub pad0: f32,
    pub vec3_val: [f32; 3],
    pub f32_val: f32,
    pub f32_val2: f32,
    pub f32_val3: f32,
    pub int_val: i32,
}

impl Default for Uniforms2 {
    fn default() -> Self {
        Self {
            vec4_val: [1.0, 0.5, 0.3, 1.0],
            vec3_val0: [0.5, 0.3, 1.0],
            pad0: 0.0,
            vec3_val: [0.5, 0.3, 1.0],
            f32_val: 1.0,
            f32_val2: 0.5,
            f32_val3: 0.3,
            int_val: 1,
        }
    }
}

pub type Uniforms2Bytes = [f32; 16];
pub type Uniforms2Ref<'a> = &'a Uniforms2Bytes;

impl AsRef<Uniforms2Bytes> for Uniforms2 {
    fn as_ref(&self) -> Uniforms2Ref {
        unsafe { mem::transmute(self) }
    }
}
