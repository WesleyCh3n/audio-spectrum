mod app;
mod binding;

/* fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let mut audio_thread = AudioThread::new();
    audio_thread.start();
    for _ in 0..30 {
        let db = audio_thread.get_decibel();
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("db len: {}", db.len());
        println!("db: {:?}", db);
    }
    audio_thread.stop();
} */

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::Vec2 { x: 400., y: 160. }),
        ..Default::default()
    };
    eframe::run_native(
        "Real Time Audio Spectrum",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
