use eframe::egui;

const RESIZE_HANDLE_SIZE: f32 = 16.0;

#[derive(
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
enum ControlType {
    Button,
    Joystick,
    SwipeArea,
    MouseArea,
}
#[derive(
    serde::Serialize,
    serde::Deserialize
)]
struct TouchForgeProfile {
    controls: Vec<CanvasControl>,
}
#[derive(
    Clone,
    serde::Serialize,
    serde::Deserialize
)]
struct CanvasControl {
    control_type: ControlType,
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
    fullscreen_editor: bool,
}

impl Default for TouchForgeApp {
    fn default() -> Self {
        Self {
            controls: Vec::new(),
            selected: None,
            fullscreen_editor: false,
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

                if ui.button("Fullscreen").clicked() {
                    self.fullscreen_editor = !self.fullscreen_editor;
                }
            });
        });

        if !self.fullscreen_editor {
            egui::SidePanel::left("controls").show(ctx, |ui| {
                ui.heading("Controls");

                let mut add_control =
                    |name: &str, control_type: ControlType, x: f32, y: f32, w: f32, h: f32| {
                        self.controls.push(CanvasControl {
                            control_type,
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
                    add_control("Button", ControlType::Button, 100.0, 100.0, 80.0, 80.0);
                }
                if ui.button("Joystick").clicked() {
                    add_control(
                        "Joystick",
                        ControlType::Joystick,
                        200.0,
                        200.0,
                        120.0,
                        120.0,
                    );
                }
                if ui.button("Swipe Area").clicked() {
                    add_control(
                        "Swipe Area",
                        ControlType::SwipeArea,
                        300.0,
                        150.0,
                        140.0,
                        100.0,
                    );
                }
                if ui.button("Mouse Area").clicked() {
                    add_control(
                        "Mouse Area",
                        ControlType::MouseArea,
                        450.0,
                        150.0,
                        140.0,
                        100.0,
                    );
                }
            });
        }

        if !self.fullscreen_editor {
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
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.fullscreen_editor {
                egui::Window::new("Editor")
                    .title_bar(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        if ui.button("Exit Fullscreen").clicked() {
                            self.fullscreen_editor = false;
                        }
                    });
            }

            let (_id, canvas_rect) = ui.allocate_space(ui.available_size());

            for (index, control) in self.controls.iter_mut().enumerate() {
                if !control.visible {
                    continue;
                }

                let rect = egui::Rect::from_min_size(
                    canvas_rect.min + egui::vec2(control.x, control.y),
                    egui::vec2(control.width, control.height),
                );

                let resize_rect = egui::Rect::from_center_size(
                    rect.right_bottom(),
                    egui::vec2(RESIZE_HANDLE_SIZE, RESIZE_HANDLE_SIZE),
                );

                let response = ui.interact(
                    rect,
                    egui::Id::new(("control", index)),
                    egui::Sense::click_and_drag(),
                );

                let resize_response = ui.interact(
                    resize_rect,
                    egui::Id::new(("resize", index)),
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

                if resize_response.dragged() && !control.lock_size {
                    let delta = resize_response.drag_delta();

                    control.width = (control.width + delta.x * 0.1).max(20.0);

                    control.height = (control.height + delta.y * 0.1).max(20.0);
                }

                let alpha = (control.opacity * 255.0) as u8;

                match control.control_type {
                    ControlType::Button => {
                        ui.painter().rect_filled(
                            rect,
                            12.0,
                            egui::Color32::from_rgba_unmultiplied(80, 120, 220, alpha),
                        );
                    }

                    ControlType::Joystick => {
                        ui.painter().circle_filled(
                            rect.center(),
                            rect.width().min(rect.height()) / 2.0,
                            egui::Color32::from_rgba_unmultiplied(80, 180, 120, alpha),
                        );

                        ui.painter().circle_filled(
                            rect.center(),
                            rect.width().min(rect.height()) / 4.0,
                            egui::Color32::WHITE,
                        );
                    }

                    ControlType::SwipeArea => {
                        ui.painter().rect_filled(
                            rect,
                            4.0,
                            egui::Color32::from_rgba_unmultiplied(220, 180, 60, alpha),
                        );
                    }

                    ControlType::MouseArea => {
                        ui.painter().rect_filled(
                            rect,
                            4.0,
                            egui::Color32::from_rgba_unmultiplied(180, 80, 220, alpha),
                        );

                        ui.painter().line_segment(
                            [rect.left_center(), rect.right_center()],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );

                        ui.painter().line_segment(
                            [rect.center_top(), rect.center_bottom()],
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                        );
                    }
                }

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

                if self.selected == Some(index) {
                    ui.painter().circle_filled(
                        resize_rect.center(),
                        RESIZE_HANDLE_SIZE / 2.0,
                        egui::Color32::YELLOW,
                    );
                }
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
