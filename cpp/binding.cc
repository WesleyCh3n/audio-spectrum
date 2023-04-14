#include "audio_thread.h"

extern "C" {

AudioThread *at_ctor() { return new AudioThread(); }
void at_dtor(AudioThread *ptr) {
  delete ptr;
  ptr = nullptr;
}
void at_start(AudioThread *ptr) { ptr->Start(); }
void at_pause(AudioThread *ptr) { ptr->Pause(); }
void at_resume(AudioThread *ptr) { ptr->Resume(); }
void at_stop(AudioThread *ptr) { ptr->Stop(); }

void at_get_freq_range(AudioThread *ptr, float *dst) { ptr->GetFreqRange(dst); }
void at_get_decibel(AudioThread *ptr, float *dst) { ptr->GetDecibel(dst); }
uint32_t at_get_decibel_len(AudioThread *ptr) { return ptr->GetDecibelLen(); }
}
