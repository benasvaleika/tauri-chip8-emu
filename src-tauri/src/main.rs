#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(non_snake_case)]

mod cpu;

use std::fs;

use cpu::CPU;
use tauri::Window;

#[tauri::command]
fn test_cpu(window: Window, rom_path: String) {
    println!("test_cpu invoked");

    let rom_contents = fs::read(rom_path).expect("Error occured while reading the file");

    let mut cpu = CPU::new();

    cpu.load_rom(&rom_contents);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_cpu])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
