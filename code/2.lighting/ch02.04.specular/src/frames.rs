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
    pub box_color: Vector3<f32>,
    pub light_color: Vector3<f32>,
    pub light_pos: Vector3<f32>,
    pub ambient_strength: f32,
    pub specular_strength: f32,
    pub shininess_strength: i32,
}

impl Default for BoxUniformWindow {
    fn default() -> Self {
        Self {
            box_color: Vector3::new(1.0, 0.5, 0.31),
            light_color: Vector3::new(1.0, 1.0, 1.0),
            light_pos: Vector3::new(-1.5, 1.5, 2.0),
            ambient_strength: 0.1,
            specular_strength: 0.5,
            shininess_strength: 32,
        }
    }
}

impl BoxUniformWindow {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Box Uniform")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Box Color:");
                    ui.color_edit_button_rgb(self.box_color.as_mut());
                });

                ui.horizontal(|ui| {
                    ui.label("Light Color:");
                    ui.color_edit_button_rgb(self.light_color.as_mut());
                });
                ui.horizontal(|ui| {
                    ui.label("Light Pos:");
                    ui.add(egui::Slider::new(&mut self.light_pos.x, -3.0..=3.0));
                    ui.add(egui::Slider::new(&mut self.light_pos.y, -3.0..=3.0));
                    ui.add(egui::Slider::new(&mut self.light_pos.z, -3.0..=3.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Ambient Strength:");
                    ui.add(egui::Slider::new(&mut self.ambient_strength, 0.01..=1.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Specular Strength:");
                    ui.add(egui::Slider::new(&mut self.specular_strength, 0.0..=1.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Shininess Strength:");
                    ui.add(egui::Slider::new(&mut self.shininess_strength, 2..=64));
                });
            });
    }
}
