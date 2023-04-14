use std::ffi::{c_float, c_void};

extern "C" {
    pub fn at_ctor() -> *mut c_void;
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
}

impl AudioThread {
    pub fn new() -> Self {
        unsafe { Self { ptr: at_ctor() } }
    }
    pub fn start(&mut self) {
        unsafe {
            at_start(self.ptr);
        }
    }
    pub fn stop(&mut self) {
        unsafe {
            at_stop(self.ptr);
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
    pub fn get_freq_range(&mut self) -> Vec<f32> {
        unsafe {
            let mut result = vec![0.0; self.get_decibel_len() as usize];
            at_get_freq_range(self.ptr, result.as_mut_ptr());
            result
        }
    }

    pub fn get_decibel_len(&mut self) -> u32 {
        unsafe { at_get_decibel_len(self.ptr) }
    }
    pub fn get_decibel(&mut self) -> Vec<f32> {
        let mut result = vec![0.0; self.get_decibel_len() as usize];
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
