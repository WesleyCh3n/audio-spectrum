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
}

impl AudioThread {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: at_ctor(50),
                is_stop: true,
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
    pub fn get_freq_range(&self) -> Vec<f32> {
        unsafe {
            let mut result = vec![0.0; self.get_decibel_len() as usize];
            at_get_freq_range(self.ptr, result.as_mut_ptr());
            println!("freq: {:?}", result);
            result
        }
    }

    pub fn get_decibel_len(&self) -> u32 {
        unsafe { at_get_decibel_len(self.ptr) }
    }
    pub fn get_decibel(&self) -> Vec<f32> {
        let mut result = vec![0.0; self.get_decibel_len() as usize];
        if self.is_stop {
            return result;
        }
        unsafe {
            at_get_decibel(self.ptr, result.as_mut_ptr());
        }
        result
            .into_iter()
            .map(|x| if x < -100.0 { 0.0 } else { x + 100. })
            .collect()
    }
}

impl Drop for AudioThread {
    fn drop(&mut self) {
        unsafe { at_dtor(self.ptr) }
    }
}
