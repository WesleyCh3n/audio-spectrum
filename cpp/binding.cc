#include "audio_thread.h"

extern "C" {

AudioThread *at_ctor(uint32_t hz_gap) { return new AudioThread(hz_gap); }
void at_dtor(AudioThread *ptr) {
  delete ptr;
  ptr = nullptr;
}
void at_start(AudioThread *ptr) { ptr->Start(); }
void at_pause(AudioThread *ptr) { ptr->Pause(); }
void at_resume(AudioThread *ptr) { ptr->Resume(); }
void at_stop(AudioThread *ptr) { ptr->Stop(); }

void at_get_freq_range(AudioThread *ptr, float *dst) { ptr->GetFreqRange(dst); }
void at_get_amplitude(AudioThread *ptr, float *dst) { ptr->GetAmplitude(dst); }
uint32_t at_get_amplitude_len(AudioThread *ptr) {
  return ptr->GetAmplitudeLen();
}

uint16_t at_get_channels(AudioThread *ptr) { return ptr->GetChannels(); };
uint32_t at_get_raw_len(AudioThread *ptr) { return ptr->GetRawLen(); }
void at_get_raw(AudioThread *ptr, float *dst, uint16_t c) {
  ptr->GetRaw(dst, c);
}
}
