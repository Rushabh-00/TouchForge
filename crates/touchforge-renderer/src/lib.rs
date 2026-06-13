use eframe::egui;

#[derive(Clone)]
struct CanvasControl {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    opacity: f32,
    visible: bool,
    lock_position: bool,
    lock_size: bool,
}

pub struct TouchForgeApp {
    controls: Vec<CanvasControl>,
    selected: Option<usize>,
}

impl Default for TouchForgeApp {
    fn default() -> Self {
        Self {
            controls: Vec::new(),
            selected: None,
        }
    }
}

impl eframe::App for TouchForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            if let Some(index) = self.selected {
                if index < self.controls.len() {
                    self.controls.remove(index);
                }
                self.selected = None;
            }
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TouchForge");

                if ui.button("New Profile").clicked() {
                    self.controls.clear();
                    self.selected = None;
                }
            });
        });

        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Controls");

            let mut add_control = |name: &str, x: f32, y: f32, w: f32, h: f32| {
                self.controls.push(CanvasControl {
                    name: name.to_string(),
                    x,
                    y,
                    width: w,
                    height: h,
                    opacity: 1.0,
                    visible: true,
                    lock_position: false,
                    lock_size: false,
                });
            };

            if ui.button("Button").clicked() {
                add_control("Button", 100.0, 100.0, 80.0, 80.0);
            }
            if ui.button("Joystick").clicked() {
                add_control("Joystick", 200.0, 200.0, 120.0, 120.0);
            }
            if ui.button("Swipe Area").clicked() {
                add_control("Swipe Area", 300.0, 150.0, 140.0, 100.0);
            }
            if ui.button("Mouse Area").clicked() {
                add_control("Mouse Area", 450.0, 150.0, 140.0, 100.0);
            }
        });

        egui::SidePanel::right("properties")
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Properties");

                if let Some(index) = self.selected {
                    if let Some(control) = self.controls.get_mut(index) {
                        ui.text_edit_singleline(&mut control.name);

                        ui.label("X");
                        ui.add(egui::DragValue::new(&mut control.x));
                        ui.add(egui::Slider::new(&mut control.x, 0.0..=3000.0));

                        ui.label("Y");
                        ui.add(egui::DragValue::new(&mut control.y));
                        ui.add(egui::Slider::new(&mut control.y, 0.0..=3000.0));

                        ui.label("Width");
                        ui.add(egui::DragValue::new(&mut control.width));
                        ui.add(egui::Slider::new(&mut control.width, 10.0..=1000.0));

                        ui.label("Height");
                        ui.add(egui::DragValue::new(&mut control.height));
                        ui.add(egui::Slider::new(&mut control.height, 10.0..=1000.0));

                        ui.collapsing("Advanced", |ui| {
                            ui.add(
                                egui::Slider::new(&mut control.opacity, 0.0..=1.0)
                                    .text("Opacity"),
                            );
                            ui.checkbox(&mut control.visible, "Visible");
                            ui.checkbox(&mut control.lock_position, "Lock Position");
                            ui.checkbox(&mut control.lock_size, "Lock Size");
                        });

                        if ui.button("Delete Control").clicked() {
                            self.controls.remove(index);
                            self.selected = None;
                        }
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (_id, canvas_rect) = ui.allocate_space(ui.available_size());

            for (index, control) in self.controls.iter_mut().enumerate() {
                if !control.visible {
                    continue;
                }

                let rect = egui::Rect::from_min_size(
                    canvas_rect.min + egui::vec2(control.x, control.y),
                    egui::vec2(control.width, control.height),
                );

                let response = ui.interact(
                    rect,
                    egui::Id::new(("control", index)),
                    egui::Sense::click_and_drag(),
                );

                if response.clicked() {
                    self.selected = Some(index);
                }

                if response.dragged() && !control.lock_position {
                    let d = response.drag_delta();

                    if response.drag_started() {
                        // optional future hook
                    }

                    control.x += d.x * 0.1;
                    control.y += d.y * 0.1;
                }

                let alpha = (control.opacity * 255.0) as u8;

                ui.painter().rect_filled(
                    rect,
                    6.0,
                    egui::Color32::from_rgba_unmultiplied(60, 60, 60, alpha),
                );

                ui.painter().rect_stroke(
                    rect,
                    6.0,
                    egui::Stroke::new(
                        if self.selected == Some(index) { 3.0 } else { 2.0 },
                        if self.selected == Some(index) {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::LIGHT_BLUE
                        },
                    ),
                    egui::StrokeKind::Middle,
                );

                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &control.name,
                    egui::FontId::proportional(16.0),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TouchForge",
        options,
        Box::new(|_| Ok(Box::new(TouchForgeApp::default()))),
    )
    .expect("failed to start TouchForge");
}
