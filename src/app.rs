use crate::binding::AudioThread;
use eframe::egui;
use eframe::egui::plot::{Bar, BarChart, Plot};

pub struct App {
    audio_thread: AudioThread,
    freq: Vec<f32>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            audio_thread: AudioThread::new(),
            freq: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Close the menu").clicked() {
                        ui.close_menu();
                    }
                });
            })
        });
        egui::SidePanel::left("left panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                match self.audio_thread.is_stop() {
                    true => ui.columns(1, |cols| {
                        if cols[0].button("Start").clicked() {
                            self.audio_thread.start();
                            self.freq = self.audio_thread.get_freq_range();
                        }
                    }),
                    false => ui.columns(1, |cols| {
                        if cols[0].button("Stop").clicked() {
                            self.audio_thread.stop();
                        }
                    }),
                }
                ui.separator();
                ui.add_enabled_ui(!self.audio_thread.is_stop(), |ui| {
                    ui.columns(2, |cols| {
                        if cols[0].button("Pause").clicked() {
                            self.audio_thread.pause();
                        }
                        if cols[1].button("Resume").clicked() {
                            self.audio_thread.resume();
                        }
                    });
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let db = self.audio_thread.get_decibel();
            Plot::new("Decibel")
                .include_y(-10.0)
                .include_y(120.0)
                .height(ui.available_height() / 2.0)
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(
                        BarChart::new(
                            self.freq
                                .iter()
                                .zip(db.iter())
                                .map(|(f, y)| Bar::new(*f as f64, *y as f64))
                                .collect(),
                        )
                        .name("db bars")
                        .color(egui::Color32::LIGHT_BLUE),
                    );
                });
            Plot::new("Picked Decibel")
                .include_y(-10.0)
                .include_y(120.0)
                .height(ui.available_height())
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(
                        BarChart::new(
                            self.freq
                                .iter()
                                .step_by(10)
                                .take(20)
                                .zip(db.iter().step_by(10).take(20))
                                .map(|(f, y)| Bar::new(*f as f64, *y as f64))
                                .collect(),
                        )
                        .name("picked db bars")
                        .color(egui::Color32::LIGHT_BLUE),
                    );
                });
            ui.ctx().request_repaint()
        });
    }
}
