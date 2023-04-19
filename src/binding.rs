use std::ffi::{c_float, c_void};

extern "C" {
    pub fn at_ctor(hz_gap: u32) -> *mut c_void;
    pub fn at_dtor(ptr: *mut c_void);
    pub fn at_start(ptr: *mut c_void);
    pub fn at_pause(ptr: *mut c_void);
    pub fn at_resume(ptr: *mut c_void);
    pub fn at_stop(ptr: *mut c_void);

    pub fn at_get_decibel(ptr: *mut c_void, dst: *mut c_float);
    pub fn at_get_decibel_len(ptr: *mut c_void) -> u32;
    pub fn at_get_freq_range(ptr: *mut c_void, dst: *mut c_float);
}

pub struct AudioThread {
    ptr: *mut c_void,
    is_stop: bool,
    prev_db: Option<Vec<f32>>,
    smooth_alpha: f32,
}

unsafe impl Sync for AudioThread {}
unsafe impl Send for AudioThread {}

impl AudioThread {
    pub fn new(hz_gap: u32) -> Self {
        unsafe {
            Self {
                ptr: at_ctor(hz_gap),
                is_stop: true,
                prev_db: None,
                smooth_alpha: 0.5,
            }
        }
    }
    pub fn is_stop(&self) -> bool {
        self.is_stop
    }
    pub fn start(&mut self) {
        unsafe {
            at_start(self.ptr);
            self.is_stop = false;
        }
    }
    pub fn stop(&mut self) {
        unsafe {
            at_stop(self.ptr);
            self.is_stop = true;
        }
    }
    pub fn pause(&mut self) {
        unsafe {
            at_pause(self.ptr);
        }
    }
    pub fn resume(&mut self) {
        unsafe {
            at_resume(self.ptr);
        }
    }
    pub fn set_smooth_alpha(&mut self, alpha: f32) {
        self.smooth_alpha = alpha;
    }
    pub fn get_freq_range(&self) -> Vec<f32> {
        unsafe {
            let mut result = vec![0.0; self.get_decibel_len() as usize];
            at_get_freq_range(self.ptr, result.as_mut_ptr());
            result
        }
    }

    pub fn get_decibel_len(&self) -> u32 {
        unsafe { at_get_decibel_len(self.ptr) }
    }
    pub fn get_decibel(&mut self) -> Vec<f32> {
        let mut result = vec![0.0; self.get_decibel_len() as usize];
        if self.is_stop {
            return result;
        }
        unsafe {
            at_get_decibel(self.ptr, result.as_mut_ptr());
        }
        let mut curr_db: Vec<f32> = result
            .into_iter()
            .map(|x| if x < -100.0 { 0.0 } else { x + 100. })
            .collect();

        if self.prev_db.is_none() {
            self.prev_db = Some(curr_db.clone())
        } else {
            let prev_db = self.prev_db.take().unwrap();
            let low_pass = prev_db
                .into_iter()
                .zip(curr_db.iter())
                .map(|(prev, curr)| {
                    (1.0 - self.smooth_alpha) * prev + self.smooth_alpha * curr
                })
                .collect();
            self.prev_db = Some(curr_db);
            curr_db = low_pass;
        }
        curr_db
    }
}

impl Default for AudioThread {
    fn default() -> Self {
        Self::new(50)
    }
}

impl Drop for AudioThread {
    fn drop(&mut self) {
        unsafe { at_dtor(self.ptr) }
    }
}
