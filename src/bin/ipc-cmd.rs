use std::sync::mpsc;
use std::{process::Command, thread};

use audio_spectrum::binding::AudioThread;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Usage: audio-ipc-cmd <timeout in millisecond>\n")
    }
    let timeout = args[1].parse().unwrap();

    let (sigint_tx, sigint_rx) = mpsc::channel::<()>();
    let (on_finish_tx, on_finish_rx) = mpsc::channel::<()>();

    thread::spawn(move || {
        let mut at = AudioThread::new();
        at.start();
        loop {
            let v: Vec<i32> = at
                .get_decibel()
                .into_iter()
                .step_by(10)
                .take(20)
                .map(|f| f.round() as i32)
                .collect();

            let mut cmd = String::from(
                "AsrISP.exe /I 0 /ID 0x26CE01C1 /CMD 0x00 0x37 0x00 0x01 0x03",
            );
            for num in v {
                cmd.push_str(format!(" 0x{:X}", num).as_str())
            }

            let output = Command::new("cmd")
                .args(["/k", cmd.as_str()])
                .output()
                .expect("failed to execute process");

            println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));

            if sigint_rx.try_recv().is_ok() {
                drop(at);
                on_finish_tx.send(()).unwrap();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(timeout));
        }
    });

    let mut msg = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut msg).unwrap();
    sigint_tx.send(()).unwrap();
    println!("Stopping! Please wait...");

    on_finish_rx.recv().unwrap();
}
