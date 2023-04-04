// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use cgmath::{perspective, Deg, InnerSpace, Matrix4, One, Point3, Vector3};
use std::mem;
use winit::dpi::PhysicalPosition;
use winit::event::{
    ElementState, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode,
    WindowEvent,
};

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
    mouse_pressed: bool,
    cursor_speed: f32,
    last_cursor_pos: PhysicalPosition<f64>,
    first_cursor_moved: bool,

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

            keyboard_speed: 0.10,
            scroll_speed: 0.12,
            mouse_pressed: false,
            cursor_speed: 0.02,
            last_cursor_pos: PhysicalPosition::new(0.0, 0.0),
            first_cursor_moved: false,

            uniform: CameraUniform::default(),
        };
        instance.update_uniform();
        instance
    }

    fn update_uniform(&mut self) {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = perspective(Deg(self.fovy), self.aspect, self.zoom_near, self.zoom_far);
        self.uniform.view_proj = OPENGL_TO_WGPU_MATRIX * proj * view;
        self.uniform.view_pos = Vector3::new(self.eye.x, self.eye.y, self.eye.z);
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
            WindowEvent::CursorMoved { position, .. } => self.process_cursor_move_event(*position),
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                self.first_cursor_moved = !self.mouse_pressed;
                true
            }
            _ => false,
        }
    }

    fn process_cursor_move_event(&mut self, position: PhysicalPosition<f64>) -> bool {
        if !self.mouse_pressed {
            return false;
        }
        if !self.first_cursor_moved {
            self.first_cursor_moved = true;
            self.last_cursor_pos = position;
        }

        let x_offset = (position.x - self.last_cursor_pos.x) as f32;
        let y_offset = (self.last_cursor_pos.y - position.y) as f32;
        self.last_cursor_pos = position;

        {
            let forward = self.target - self.eye;
            let forward_norm = forward.normalize();
            let forward_mag = forward.magnitude();
            let right = forward_norm.cross(self.up);
            self.eye = self.target
                - (forward + x_offset * right * self.cursor_speed).normalize() * forward_mag;
        }

        {
            let forward = self.target - self.eye;
            let forward_mag = forward.magnitude();
            self.eye = self.target
                - (forward + y_offset * self.up * self.cursor_speed).normalize() * forward_mag;
        }
        self.update_uniform();
        true
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
            MouseScrollDelta::PixelDelta(pos) => {
                // TODO(Shaohua): rotate view
                let forward = self.target - self.eye;
                let forward_norm = forward.normalize();
                self.eye += (forward_norm * self.scroll_speed) * pos.y as f32;
                self.update_uniform();
                true
            }
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
    pub view_pos: Vector3<f32>,
    pad: f32,
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Matrix4::one(),
            view_pos: Vector3::new(0.0, 0.0, 0.0),
            pad: 0.0,
        }
    }
}

pub type CameraUniformBytes = [f32; 20];
pub type CameraUniformRef<'a> = &'a CameraUniformBytes;

impl AsRef<CameraUniformBytes> for CameraUniform {
    fn as_ref(&self) -> CameraUniformRef {
        unsafe { mem::transmute(self) }
    }
}
