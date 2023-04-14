use crate::binding::AudioThread;
use eframe::egui;
use eframe::egui::plot::{Bar, BarChart, Plot};

pub struct App {
    audio_thread: AudioThread,
    freq: Vec<f32>,
    running: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            audio_thread: AudioThread::new(),
            freq: Vec::new(),
            running: false,
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
                ui.columns(2, |cols| {
                    if cols[0].button("Start").clicked() {
                        self.audio_thread.start();
                        self.running = true;
                        self.freq = self.audio_thread.get_freq_range();
                    }
                    if cols[1].button("Stop").clicked() {
                        self.audio_thread.stop();
                        self.running = false;
                    }
                });
                ui.columns(2, |cols| {
                    if cols[0].button("Pause").clicked() {
                        self.audio_thread.pause();
                    }
                    if cols[1].button("Resume").clicked() {
                        self.audio_thread.resume();
                    }
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("Decibel").include_y(-10.0).include_y(120.0).show(
                ui,
                |plot_ui| {
                    if !self.running {
                        return;
                    }
                    let db = self.audio_thread.get_decibel();
                    let bars = BarChart::new(
                        self.freq
                            .iter()
                            .zip(db.iter())
                            .map(|(f, y)| Bar::new(*f as f64, *y as f64))
                            .collect(),
                    )
                    .name("db bar")
                    .width(100.0)
                    .color(egui::Color32::LIGHT_BLUE);
                    plot_ui.bar_chart(bars);
                    plot_ui.ctx().request_repaint()
                },
            );
        });
    }
}
