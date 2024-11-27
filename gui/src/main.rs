#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod config;
mod app;



fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config::GUI_WIDTH, config::GUI_HEIGHT])
            .with_resizable(false),
        multisampling: 2,
        centered: true,

        ..Default::default()
    };

    eframe::run_native(
        config::APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.set_style(egui::Style {
                spacing: egui::Spacing {
                    slider_width: config::GUI_WIDTH * 0.75,
                    ..Default::default()
                },
                ..Default::default()
            });
            Ok(Box::new(app::VolumeMixerApp::new(cc)))
        }),
    )
}
