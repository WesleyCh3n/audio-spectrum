use audio_spectrum::Worker;

use eframe::egui;
use eframe::egui::plot::{Bar, BarChart, Legend, Plot};
use eframe::epaint::{Color32, Stroke};

pub struct App {
    worker: Worker,
    decibel: Vec<f32>,
    raws: Vec<Vec<f32>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            worker: Worker::new(cc.egui_ctx.clone()),
            decibel: Vec::new(),
            raws: Vec::new(),
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
                ui.add_enabled_ui(self.worker.is_stop(), |ui| {
                    ui.heading("Parameter");
                    egui::Grid::new("Utility")
                        .striped(true)
                        .num_columns(2)
                        .min_col_width(100.)
                        .show(ui, |ui| {
                            ui.label("Timout");
                            ui.add(
                                egui::DragValue::new(&mut self.worker.timeout)
                                    .clamp_range(20.0..=1000.0)
                                    .speed(1.0),
                            );
                            ui.end_row();
                            ui.label("Freq. Gap");
                            ui.add(
                                egui::DragValue::new(&mut self.worker.hz_gap)
                                    .clamp_range(1.0..=10000.0)
                                    .speed(10.0),
                            );
                            ui.end_row();
                            ui.label("Smooth Alpha");
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.worker.smooth_alpha,
                                )
                                .clamp_range(0.1..=1.0)
                                .speed(0.1),
                            );
                            ui.end_row();
                            if ui.button("Reset").clicked() {
                                self.worker.timeout = 50;
                                self.worker.hz_gap = 50;
                                self.worker.smooth_alpha = 0.5;
                            }
                            ui.end_row();
                        });
                });
                ui.separator();
                match self.worker.is_stop() {
                    true => ui.columns(1, |cols| {
                        if cols[0].button("Start").clicked() {
                            self.worker.start();
                        }
                    }),
                    false => ui.columns(1, |cols| {
                        if cols[0].button("Stop").clicked() {
                            self.worker.stop();
                        }
                    }),
                }
                ui.separator();
                ui.add_enabled_ui(!self.worker.is_stop(), |ui| {
                    ui.columns(2, |cols| {
                        if cols[0].button("Pause").clicked() {
                            self.worker.pause();
                        }
                        if cols[1].button("Resume").clicked() {
                            self.worker.resume();
                        }
                    });
                })
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(rx) = self.worker.db_rx.as_ref() {
                if let Ok(db) = rx.try_recv() {
                    self.decibel = db
                }
            }
            if let Some(rx) = self.worker.raw_rx.as_ref() {
                if let Ok(raws) = rx.try_recv() {
                    self.raws = raws
                }
            }
            let Self {
                decibel,
                raws,
                worker,
                ..
            } = &self;
            let Worker { freq, .. } = worker;
            Plot::new("Decibel")
                .include_y(-10.0)
                .include_y(100.0)
                .height(ui.available_height() / 3.0)
                .show(ui, |plot_ui| {
                    if let Some(f) = freq.as_ref() {
                        plot_decibel(plot_ui, f, decibel, None, 1.0);
                    }
                });
            Plot::new("Picked Decibel")
                .include_y(-10.0)
                .include_y(100.0)
                .height(ui.available_height() / 2.0)
                .show(ui, |plot_ui| {
                    if let Some(f) = freq.as_ref() {
                        plot_decibel(plot_ui, f, decibel, Some((10, 20)), 100.0)
                    }
                });
            Plot::new("Raw Data")
                .include_y(1.0)
                .include_y(-1.0)
                .legend(Legend::default())
                .height(ui.available_height())
                .show(ui, |plot_ui| {
                    raws.iter().enumerate().for_each(|(c, raw)| {
                        let points: egui::plot::PlotPoints = raw
                            .iter()
                            .enumerate()
                            .map(|(x, y)| [x as f64, *y as f64])
                            .collect();
                        plot_ui.line(
                            egui::plot::Line::new(points)
                                .name(format!("Channel {c}")),
                        );
                    });
                });
        });
    }
}

fn plot_decibel(
    ui: &mut egui::plot::PlotUi,
    freq: &[f32],
    db: &[f32],
    step_take: Option<(usize, usize)>,
    width: f64,
) {
    let freq = freq.iter();
    let db = db.iter();

    let color_palette = |x: u32| match x {
        0..=30 => Color32::from_rgb(151, 203, 255),
        31..=40 => Color32::from_rgb(110, 169, 255),
        41..=50 => Color32::from_rgb(76, 135, 255),
        51..=60 => Color32::from_rgb(56, 106, 255),
        61.. => Color32::from_rgb(32, 77, 226),
    };
    let my_bar = |(x, y): (&f32, &f32)| {
        Bar::new(*x as f64, *y as f64)
            .stroke(Stroke::new(1.0, color_palette((*y).round() as u32)))
            .fill(color_palette((*y).round() as u32))
            .width(width)
    };

    ui.bar_chart(
        BarChart::new(match step_take {
            Some((step, take)) => {
                freq.zip(db).map(my_bar).step_by(step).take(take).collect()
            }
            None => freq.zip(db).map(my_bar).collect(),
        })
        .name("picked db bars")
        .color(egui::Color32::LIGHT_BLUE),
    );
}
