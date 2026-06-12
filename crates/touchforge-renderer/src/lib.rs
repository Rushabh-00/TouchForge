use eframe::egui;

pub struct TouchForgeApp;

impl eframe::App for TouchForgeApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TouchForge");

                if ui.button("New Profile").clicked() {
                    println!("New Profile");
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
                    println!("Add Button");
                }

                if ui.button("Joystick").clicked() {
                    println!("Add Joystick");
                }

                if ui.button("Swipe Area").clicked() {
                    println!("Add Swipe Area");
                }

                if ui.button("Mouse Area").clicked() {
                    println!("Add Mouse Area");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Canvas");

            ui.separator();

            ui.label("TouchForge Editor");

            ui.label("Profile editing canvas will appear here.");

            ui.add_space(20.0);

            let available = ui.available_size();

            let (_id, rect) = ui.allocate_space(available);

            ui.painter().rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::GRAY),
                egui::StrokeKind::Middle,
            );

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Canvas",
                egui::FontId::proportional(24.0),
                egui::Color32::LIGHT_GRAY,
            );
        });
    }
}

pub fn run() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TouchForge",
        options,
        Box::new(|_| Ok(Box::new(TouchForgeApp))),
    )
    .expect("failed to start TouchForge");
}
