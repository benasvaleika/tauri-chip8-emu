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
use serde::Deserialize;
use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct Payload {
    #[serde(serialize_with = "<[_]>::serialize")]
    display: [u8; 2048],
}

#[derive(Debug, Deserialize)]
struct KeyChange {
    keyValue: u8,
}

#[tauri::command]
async fn start_cpu(window: Window, rom_path: String) {
    let mut cpu = CPU::new();

    let rom_contents = fs::read(rom_path).expect("Error occured while reading the file");

    cpu.load_rom(&rom_contents);

    window.listen_global("key-change", |event| {
        let payload: KeyChange =
            serde_json::from_str(event.payload().unwrap()).expect("JSON was not well-formatted");

        if (payload.keyValue >= 0) && (payload.keyValue <= 15) {
            println!("pressed")
        } else {
            println!("cleaned")
        }
    });

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
