// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod process;
use process::{enum_proc, Process, ProcessItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn proccc() {
    println!("MyCheatEngine");
    println!("all process:");
    let pids = enum_proc()
        .unwrap()
        .into_iter()
        .flat_map(Process::open)
        .flat_map(|proc| match proc.name() {
            Ok(name) => Ok(ProcessItem {
                pid: proc.pid(),
                name,
            }),
            Err(err) => Err(err),
        })
        .collect::<Vec<_>>();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, proccc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
