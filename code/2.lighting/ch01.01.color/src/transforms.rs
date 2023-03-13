// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use cgmath::{BaseFloat, Matrix4, Vector3};

pub fn scale<S: BaseFloat>(mat: &Matrix4<S>, scaling: Vector3<S>) -> Matrix4<S> {
    let scale_mat = Matrix4::<S>::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
    mat * scale_mat
}
