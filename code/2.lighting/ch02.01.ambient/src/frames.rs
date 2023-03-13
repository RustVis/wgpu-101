// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use cgmath::Vector3;
use instant::Instant;

#[derive(Debug, Clone)]
pub struct UserWindow {
    name: String,
    age: u32,
}

impl Default for UserWindow {
    fn default() -> Self {
        Self {
            name: "Author".to_owned(),
            age: 42,
        }
    }
}

impl UserWindow {
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn age(&self) -> u32 {
        self.age
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("User")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let name_label = ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name)
                        .labelled_by(name_label.id);
                });
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                if ui.button("Click each year").clicked() {
                    self.age += 1;
                }
                ui.label(format!("Hello '{}', age {}", self.name, self.age));
            });
    }
}

#[derive(Debug, Clone)]
pub struct ColorWindow {
    color: Vector3<f32>,
}

impl Default for ColorWindow {
    fn default() -> Self {
        Self {
            color: Vector3::new(0.3, 0.4, 0.5),
        }
    }
}

impl ColorWindow {
    pub fn set_color(&mut self, color: Vector3<f32>) {
        self.color = color;
    }

    pub fn color(&self) -> &Vector3<f32> {
        &self.color
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Color")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("Select vertex color:");
                ui.color_edit_button_rgb(self.color.as_mut());
            });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FpsWindow {
    fps: u32,
    frames: u32,
    timer: Instant,
}

impl Default for FpsWindow {
    fn default() -> Self {
        Self {
            fps: 0,
            frames: 0,
            timer: Instant::now(),
        }
    }
}

impl FpsWindow {
    pub fn update(&mut self) {
        let dt = self.timer.elapsed().as_secs_f64();
        let fps = (f64::from(self.frames) / dt).round() as u32;
        //log::info!("fps: {fps}");
        if dt > 1.0 {
            self.frames = 0;
            self.timer = Instant::now();
            self.fps = fps;
        }
        self.frames += 1;
    }

    #[must_use]
    pub const fn fps(&self) -> u32 {
        self.fps
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("FPS")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading(format!("FPS: {}", self.fps));
            });
    }
}

#[derive(Debug, Clone)]
pub struct BoxUniformWindow {
    light_color: Vector3<f32>,
    ambient: f32,
}

impl Default for BoxUniformWindow {
    fn default() -> Self {
        Self {
            light_color: Vector3::new(0.3, 0.4, 0.5),
            ambient: 0.9,
        }
    }
}

impl BoxUniformWindow {
    pub fn set_color(&mut self, color: Vector3<f32>) {
        self.light_color = color;
    }

    pub fn color(&self) -> &Vector3<f32> {
        &self.light_color
    }

    pub fn set_ambient(&mut self, ambient: f32) {
        self.ambient = ambient;
    }

    pub const fn ambient(&self) -> f32 {
        self.ambient
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Box Uniform")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Light Color:");
                    ui.color_edit_button_rgb(self.light_color.as_mut());
                });
                ui.horizontal(|ui| {
                    ui.label("Ambient Strength:");
                    ui.add(egui::Slider::new(&mut self.ambient, 0.0..=1.0));
                });
            });
    }
}
