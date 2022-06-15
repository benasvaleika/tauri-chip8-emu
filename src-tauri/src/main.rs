#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

mod chip8_font;
mod cpu;

use core::time;
use std::{fs, thread};

use cpu::CPU;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    #[serde(serialize_with = "<[_]>::serialize")]
    display: [u8; 2048],
}

#[tauri::command]
async fn start_cpu(window: Window, rom_path: String) {
    let rom_contents = fs::read(rom_path).expect("Error occured while reading the file");

    let mut cpu = CPU::new();

    cpu.load_rom(&rom_contents);

    loop {
        cpu.emulate_cycle();

        if cpu.display_changed {
            window
                .emit(
                    "display-update",
                    Payload {
                        display: cpu.display,
                    },
                )
                .unwrap();
            println!("Display emitted");
        }

        thread::sleep(time::Duration::from_millis(2));
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_cpu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
