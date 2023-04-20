pub mod binding;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Receiver,
    Arc, Mutex,
};

use binding::AudioThread;

pub struct Worker {
    handle: Option<std::thread::JoinHandle<()>>,
    is_stop: Arc<AtomicBool>,
    is_pause: Arc<AtomicBool>,

    audio_thread: Option<Arc<Mutex<AudioThread>>>,
    pub timeout: u64,
    pub hz_gap: u32,
    pub smooth_alpha: f32,

    pub freq: Option<Vec<f32>>,
    pub db_rx: Option<Receiver<Vec<f32>>>,
    pub raw_rx: Option<Receiver<Vec<Vec<f32>>>>,

    ctx: eframe::egui::Context,
}

impl Worker {
    pub fn new(ctx: eframe::egui::Context) -> Self {
        Self {
            handle: None,
            is_stop: Arc::new(AtomicBool::new(true)),
            is_pause: Arc::new(AtomicBool::new(true)),

            audio_thread: None,
            timeout: 50,
            hz_gap: 50,
            smooth_alpha: 0.5,
            freq: None,
            db_rx: None,
            raw_rx: None,

            ctx,
        }
    }
    pub fn start(&mut self) {
        let (db_tx, db_rx) = std::sync::mpsc::channel();
        let (raw_tx, raw_rx) = std::sync::mpsc::channel();
        self.db_rx = Some(db_rx);
        self.raw_rx = Some(raw_rx);

        self.is_stop.store(false, Ordering::SeqCst);
        self.is_pause.store(false, Ordering::SeqCst);
        let is_stop = self.is_stop.clone();
        let is_pause = self.is_pause.clone();

        self.audio_thread =
            Some(Arc::new(Mutex::new(AudioThread::new(self.hz_gap))));
        let audio_thread = self.audio_thread.as_mut().unwrap().clone();

        audio_thread
            .lock()
            .unwrap()
            .set_smooth_alpha(self.smooth_alpha);

        let ctx = self.ctx.clone();
        let timeout = self.timeout;
        self.freq = Some(audio_thread.lock().unwrap().get_freq_range());
        self.handle = Some(std::thread::spawn(move || {
            audio_thread.lock().unwrap().start();
            loop {
                if is_stop.load(Ordering::SeqCst) {
                    audio_thread.lock().unwrap().stop();
                    break;
                }
                if !is_pause.load(Ordering::SeqCst) {
                    db_tx
                        .send(audio_thread.lock().unwrap().get_decibel())
                        .unwrap_or_default();

                    raw_tx
                        .send(audio_thread.lock().unwrap().get_raw())
                        .unwrap_or_default();
                    ctx.request_repaint();
                }
                std::thread::sleep(std::time::Duration::from_millis(timeout));
            }
        }));
    }
    pub fn stop(&mut self) {
        self.is_stop.store(true, Ordering::SeqCst);
        if self.handle.is_some() {
            self.handle.take().unwrap().join().unwrap();
        }
    }
    pub fn pause(&mut self) {
        self.audio_thread.as_mut().unwrap().lock().unwrap().pause();
        self.is_pause.store(true, Ordering::SeqCst)
    }
    pub fn resume(&mut self) {
        self.audio_thread.as_mut().unwrap().lock().unwrap().resume();
        self.is_pause.store(false, Ordering::SeqCst)
    }
    pub fn is_stop(&self) -> bool {
        self.is_stop.load(Ordering::SeqCst)
    }
    pub fn get_freq_range(&self) -> Vec<f32> {
        self.audio_thread
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .get_freq_range()
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}
