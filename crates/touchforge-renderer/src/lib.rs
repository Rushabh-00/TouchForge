use eframe::egui;

#[derive(Clone)]
struct CanvasControl {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
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

                if ui.button("Open").clicked() {
                    println!("Open");
                }

                if ui.button("Import ICP").clicked() {
                    println!("Import ICP");
                }

                if ui.button("Export").clicked() {
                    println!("Export");
                }
            });
        });

        egui::SidePanel::left("controls")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Controls");

                ui.separator();

                if ui.button("Button").clicked() {
                    self.controls.push(CanvasControl {
                        name: "Button".to_string(),
                        x: 100.0,
                        y: 100.0,
                        width: 80.0,
                        height: 80.0,
                    });
                }

                if ui.button("Joystick").clicked() {
                    self.controls.push(CanvasControl {
                        name: "Joystick".to_string(),
                        x: 200.0,
                        y: 200.0,
                        width: 120.0,
                        height: 120.0,
                    });
                }

                if ui.button("Swipe Area").clicked() {
                    self.controls.push(CanvasControl {
                        name: "Swipe Area".to_string(),
                        x: 300.0,
                        y: 150.0,
                        width: 140.0,
                        height: 100.0,
                    });
                }

                if ui.button("Mouse Area").clicked() {
                    self.controls.push(CanvasControl {
                        name: "Mouse Area".to_string(),
                        x: 450.0,
                        y: 150.0,
                        width: 140.0,
                        height: 100.0,
                    });
                }

                ui.separator();
                ui.label(format!("Controls: {}", self.controls.len()));

                if let Some(index) = self.selected {
                    if let Some(control) = self.controls.get(index) {
                        ui.separator();
                        ui.label(format!("Selected: {}", control.name));
                    }
                }
            });

        egui::SidePanel::right("properties")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Properties");

                if let Some(index) = self.selected {
                    if let Some(control) = self.controls.get_mut(index) {
                        ui.separator();

                        ui.label("Name");
                        ui.text_edit_singleline(&mut control.name);

                        ui.separator();

                        ui.label("Position");

                        ui.horizontal(|ui| {
                            ui.label("X");
                            ui.add(
                                egui::DragValue::new(&mut control.x)
                                    .speed(1.0),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Y");
                            ui.add(
                                egui::DragValue::new(&mut control.y)
                                    .speed(1.0),
                            );
                        });

                        ui.separator();

                        ui.label("Size");

                        ui.horizontal(|ui| {
                            ui.label("Width");
                            ui.add(
                                egui::DragValue::new(&mut control.width)
                                    .speed(1.0)
                                    .range(10.0..=2000.0),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Height");
                            ui.add(
                                egui::DragValue::new(&mut control.height)
                                    .speed(1.0)
                                    .range(10.0..=2000.0),
                            );
                        });

                        ui.separator();

                        if ui.button("Delete Control").clicked() {
                            self.controls.remove(index);
                            self.selected = None;
                        }
                    }
                } else {
                    ui.separator();
                    ui.label("Select a control");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Canvas");
            ui.separator();

            let available = ui.available_size();
            let (_id, canvas_rect) = ui.allocate_space(available);

            ui.painter().rect_stroke(
                canvas_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::GRAY),
                egui::StrokeKind::Middle,
            );

            for (index, control) in self.controls.iter_mut().enumerate() {
                let control_rect = egui::Rect::from_min_size(
                    canvas_rect.min + egui::vec2(control.x, control.y),
                    egui::vec2(control.width, control.height),
                );

                let response = ui.interact(
                    control_rect,
                    egui::Id::new(("control", index)),
                    egui::Sense::click_and_drag(),
                );

                if response.clicked() {
                    self.selected = Some(index);
                }

                if response.dragged() {
                    let delta = response.drag_delta();

                    control.x += delta.x;
                    control.y += delta.y;
                }

                ui.painter().rect_filled(
                    control_rect,
                    4.0,
                    egui::Color32::from_gray(60),
                );

                ui.painter().rect_stroke(
                    control_rect,
                    4.0,
                    egui::Stroke::new(
                        if self.selected == Some(index) {
                            3.0
                        } else {
                            2.0
                        },
                        if self.selected == Some(index) {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::LIGHT_BLUE
                        },
                    ),
                    egui::StrokeKind::Middle,
                );

                ui.painter().text(
                    control_rect.center(),
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
