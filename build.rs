#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=./cpp/*");
    cc::Build::new()
        .cpp(true)
        .include("cpp")
        .include("cpp/audio_thread")
        .include("cpp/audio_thread/libfft")
        .file("./cpp/binding.cc")
        .file("./cpp/audio_thread/audio_thread.cc")
        .file("./cpp/audio_thread/libfft/kiss_fft.c")
        .file("./cpp/audio_thread/libfft/kiss_fftr.c")
        .flag_if_supported("/std:c++17")
        .cpp_link_stdlib("ole32")
        .flag_if_supported("/EHsc")
        .warnings_into_errors(true)
        .compile("audio_thread");
}
