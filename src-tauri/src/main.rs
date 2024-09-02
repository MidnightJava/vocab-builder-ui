// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fix_path_env::fix;
// use objc::{class, msg_send, runtime};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tauri::WindowEvent;

fn kill_process(pid: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        println!("Sending INT signal to process with PID: {}", pid);

        let mut kill = Command::new("kill").args(["-s", "SIGINT", &pid]).spawn()?;
        kill.wait()?;
    }

    #[cfg(windows)]
    {
        println!("Sending taskkill to process with PID: {}", pid);

        let mut kill = Command::new("taskkill")
            .args(["/PID", &pid, "/F"])
            .spawn()?;
        kill.wait()?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = fix() {
        println!("{}", e);
    } else {
        let os: &str = env::consts::OS;
        let current_dir = env::current_dir().unwrap();
        let binary_path: PathBuf;
        if cfg!(debug_assertions) {
            //TODO don't hardcode target triple
            binary_path = current_dir.join("binaries/server-x86_64-unknown-linux-gnu");
        } else {
            if os == "linux" {
                binary_path = Path::new("server").to_path_buf();
            } else if os == "macos" {
                let current_exe =
                    env::current_exe().expect("Failed to get current executable path");

                let bundle_path: PathBuf = current_exe
                    .parent() // MacOS directory
                    .and_then(|p| p.parent()) // Contents directory
                    .and_then(|p| p.parent()) // .app directory
                    .expect("Failed to get bundle path")
                    .to_path_buf();

                binary_path = bundle_path.join("Contents/MacOS/server");
                println!("Bundle path: {}", bundle_path.display());
                println!("Binary path: {}", binary_path.display());
            } else if os == "windows" {
                binary_path = Path::new("TBD").to_path_buf();
            } else {
                binary_path = Path::new("").to_path_buf();
            }
        }
        println!("Binary path: {:?}", binary_path);

        let child = Command::new(binary_path.as_path())
            .spawn()
            .expect("Failed to start process");

        tauri::Builder::default()
            .on_window_event(move |event| match event.event() {
                WindowEvent::Destroyed { .. } => match kill_process(&child.id().to_string()) {
                    Ok(_) => println!("Process killed successfully."),
                    Err(e) => eprintln!("Failed to kill process: {}", e),
                },
                _ => {}
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
