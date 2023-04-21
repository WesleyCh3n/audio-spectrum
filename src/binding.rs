use std::ffi::{c_float, c_void};

extern "C" {
    pub fn at_ctor(hz_gap: u32) -> *mut c_void;
    pub fn at_dtor(ptr: *mut c_void);
    pub fn at_start(ptr: *mut c_void);
    pub fn at_pause(ptr: *mut c_void);
    pub fn at_resume(ptr: *mut c_void);
    pub fn at_stop(ptr: *mut c_void);

    pub fn at_get_amplitude(ptr: *mut c_void, dst: *mut c_float);
    pub fn at_get_amplitude_len(ptr: *mut c_void) -> u32;
    pub fn at_get_freq_range(ptr: *mut c_void, dst: *mut c_float);

    pub fn at_get_channels(ptr: *mut c_void) -> u16;
    pub fn at_get_raw_len(ptr: *mut c_void) -> u32;
    pub fn at_get_raw(ptr: *mut c_void, dst: *mut c_float, channel: u16);
}

pub struct AudioThread {
    ptr: *mut c_void,
    is_stop: bool,
    prev_am: Option<Vec<f32>>,
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
                prev_am: None,
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
            let mut result = vec![0.0; self.get_am_len() as usize];
            at_get_freq_range(self.ptr, result.as_mut_ptr());
            result
        }
    }

    pub fn get_am_len(&self) -> u32 {
        unsafe { at_get_amplitude_len(self.ptr) }
    }
    pub fn get_am(&mut self) -> Vec<f32> {
        let mut result = vec![0.0; self.get_am_len() as usize];
        if self.is_stop {
            return result;
        }
        unsafe {
            at_get_amplitude(self.ptr, result.as_mut_ptr());
        }
        let mut curr_am: Vec<f32> = result
            .into_iter()
            /* .map(|x| {
                (if x + 120.0 <= 0.0 { 0.0 } else { x + 120.0 }).powi(2) / 100.0
            }) */
            .map(|x| {
                let v = x.sqrt() * 1000.0;
                if v > 128.0 {
                    128.0
                } else {
                    v
                }
            })
            .collect();

        if self.prev_am.is_none() {
            self.prev_am = Some(curr_am.clone())
        } else {
            let prev_am = self.prev_am.take().unwrap();
            let low_pass_am = prev_am
                .into_iter()
                .zip(curr_am.iter())
                .map(|(prev, curr)| {
                    (1.0 - self.smooth_alpha) * prev + self.smooth_alpha * curr
                })
                .collect();
            self.prev_am = Some(curr_am);
            curr_am = low_pass_am;
        }
        curr_am
    }

    fn at_get_channels(&self) -> u16 {
        unsafe { at_get_channels(self.ptr) }
    }
    fn at_get_raw_len(&self) -> u32 {
        unsafe { at_get_raw_len(self.ptr) }
    }
    pub fn get_raw(&mut self) -> Vec<Vec<f32>> {
        let mut result = vec![
            vec![0.0; self.at_get_raw_len() as usize];
            self.at_get_channels() as usize
        ];

        unsafe {
            for c in 0..self.at_get_channels() {
                at_get_raw(self.ptr, result[c as usize].as_mut_ptr(), c)
            }
        }
        result
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
