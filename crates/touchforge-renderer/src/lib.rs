use eframe::egui;

pub struct TouchForgeApp;

impl eframe::App for TouchForgeApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TouchForge");

            ui.separator();

            ui.label("TouchForge Renderer");
            ui.label("v0.2-alpha");

            if ui.button("Create Profile").clicked() {
                println!("Create Profile");
            }

            if ui.button("Open Profile").clicked() {
                println!("Open Profile");
            }

            if ui.button("Import ICP").clicked() {
                println!("Import ICP");
            }

            if ui.button("Export Profile").clicked() {
                println!("Export Profile");
            }
        });
    }
}

pub fn run() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "TouchForge",
        options,
        Box::new(|_| Ok(Box::new(TouchForgeApp))),
    )
    .expect("failed to start TouchForge");
}
