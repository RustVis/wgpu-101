// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use cgmath::{BaseFloat, Matrix4, Rad, Vector3};

pub fn translate<S: BaseFloat>(mat: &mut Matrix4<S>, translation: Vector3<S>) {
    let trans_mat = Matrix4::from_translation(translation);
    *mat = *mat * trans_mat
}

pub fn rotate<S: BaseFloat>(mat: &mut Matrix4<S>, rotation: Vector3<S>) {
    let rotate_mat_x = Matrix4::from_angle_x(Rad(rotation[0]));
    let rotate_mat_y = Matrix4::from_angle_y(Rad(rotation[1]));
    let rotate_mat_z = Matrix4::from_angle_z(Rad(rotation[2]));

    *mat = *mat * rotate_mat_z * rotate_mat_y * rotate_mat_x
}

pub fn scale<S: BaseFloat>(mat: &mut Matrix4<S>, scaling: Vector3<S>) {
    let scale_mat = Matrix4::<S>::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);
    *mat = *mat * scale_mat
}
