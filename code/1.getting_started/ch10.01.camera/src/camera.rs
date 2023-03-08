// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::{perspective, Deg, InnerSpace, Matrix4, One, Point3, Vector3};
use std::mem;
use winit::event::{KeyboardInput, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent};

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

    keyboard_speed: f32,
    scroll_speed: f32,

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

            keyboard_speed: 0.05,
            scroll_speed: 0.08,

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

    pub fn uniform_ref(&self) -> CameraUniformRef {
        self.uniform.as_ref()
    }

    pub fn process_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => self.process_key_event(*keycode),
            WindowEvent::MouseWheel {
                delta,
                phase: TouchPhase::Moved,
                ..
            } => self.process_wheel_event(*delta),
            _ => false,
        }
    }

    fn process_wheel_event(&mut self, delta: MouseScrollDelta) -> bool {
        match delta {
            MouseScrollDelta::LineDelta(_horizontal, vertical) => {
                let forward = self.target - self.eye;
                let forward_norm = forward.normalize();
                self.eye += (forward_norm * self.scroll_speed) * vertical;
                self.update_uniform();
                true
            }
            _ => false,
        }
    }

    fn process_key_event(&mut self, keycode: VirtualKeyCode) -> bool {
        let forward = self.target - self.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();
        let right = forward_norm.cross(self.up);

        match keycode {
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.eye =
                    self.target - (forward - right * self.keyboard_speed).normalize() * forward_mag;
                self.update_uniform();
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.eye =
                    self.target - (forward + right * self.keyboard_speed).normalize() * forward_mag;
                self.update_uniform();
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.eye -= forward_norm * self.keyboard_speed;
                self.update_uniform();
                true
            }
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.eye += forward_norm * self.keyboard_speed;
                self.update_uniform();
                true
            }
            _ => false,
        }
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
