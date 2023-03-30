// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

use cgmath::Vector3;

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
            color: Vector3::new(1.0, 0.5, 0.3),
        }
    }
}

impl ColorWindow {
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
