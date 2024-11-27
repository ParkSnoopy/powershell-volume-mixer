use crate::config;

use powershell_audio_core::{
    Audio,
};

use egui::{
    Vec2,

    Context,
    CentralPanel,
    RichText,
    Grid,
    Label,
    Button,
    Slider,
};

use std::time::Instant;



pub struct VolumeMixerApp {
    audio: Audio,

    mute_unmute_btn_label: String,
    local_volume_step: u8,

    last_audio_update: Instant,
}

impl Default for VolumeMixerApp {
    fn default() -> Self {
        let audio = Audio::default();
        Self {
            audio: audio.clone(),

            mute_unmute_btn_label: ( if audio.is_muted() {"Unmute"} else {"Mute"} ).to_string(),
            local_volume_step: audio.get_volume(),

            last_audio_update: Instant::now(),
        }
    }
}

impl VolumeMixerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for VolumeMixerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::global_theme_preference_buttons(ui);

            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(config::APP_NAME).size(24f32)
                );
            });

            ui.separator();

            Grid::new("volume_slider").show(ui, |ui| {
                ui.add_sized(
                    ui.available_size() * Vec2{x:1.0, y:0.5},
                    Label::new("Volume")
                );
                ui.add_sized(
                    ui.available_size() * Vec2{x:1.0, y:0.5},
                    Slider::new(&mut self.local_volume_step, 0..=100)
                        .smart_aim(true)
                );
                ui.end_row();
            });

            ui.separator();

            if ui.add_sized(
                ui.available_size(),
                Button::new(&self.mute_unmute_btn_label),
            ).clicked() {
                if self.audio.is_muted() {
                    self.audio.unmute().unwrap();
                    self.mute_unmute_btn_label = "Mute".to_string();
                } else {
                    self.audio.mute().unwrap();
                    self.mute_unmute_btn_label = "Unmute".to_string();
                }
            };

            ui.separator();
        });

        if self.last_audio_update.elapsed().subsec_millis() > config::COOLDOWN {
            if self.local_volume_step != self.audio.get_volume() {
                self.audio.set_volume(self.local_volume_step).unwrap();
                self.last_audio_update = Instant::now();
            };
        };
    }
}
