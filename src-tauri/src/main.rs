// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger;
use log::info;
use std::env;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;
use tauri::{LogicalSize, Size};

fn kill_process(pid: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        info!("Sending INT signal to process with PID: {}", pid);

        //There are two server processes. We kill the child first, then the parent
        let mut kill = Command::new("kill")
            .args(["-s", "SIGINT", "-P", &pid])
            .spawn()?;
        kill.wait()?;
        let mut kill = Command::new("kill").args(["-s", "SIGINT", &pid]).spawn()?;
        kill.wait()?;

        //On MacOS, if the user quits the app without closing the app window, the servers will not be killed.
        //So we kill all server proceeses by name when they eventually close the app window.
        let mut kill = Command::new("pkill").args(["vb_server"]).spawn()?;
        kill.wait()?;
    }

    #[cfg(windows)]
    {
        info!("Sending taskkill to process with PID: {}", pid);

        // Kill the parent and child process
        let mut kill = Command::new("taskkill")
            .args(["/T", "/F", "/PID", &pid])
            .spawn()?;
        kill.wait()?;

        //Just in case, let's kill the server by name as well
        let mut kill = Command::new("taskkill")
            .args(["/F", "/IM", "vb_server.exe"])
            .spawn()?;
        kill.wait()?;
    }

    Ok(())
}

fn main() {
    env_logger::init();
    info!("Logging initialized");
    let os: &str = env::consts::OS;
    let binary_path: PathBuf;

    if cfg!(debug_assertions) {
        if os == "windows" {
            binary_path = Path::new("binaries/vb_server-x86_64-pc-windows-msvc.exe").to_path_buf();
        } else if os == "linux" {
            binary_path = Path::new("binaries/vb_server-x86_64-unknown-linux-gnu").to_path_buf();
        } else {
            // Dev build not currently supported on macos
            binary_path = Path::new("").to_path_buf();
        }
    } else {
        if os == "linux" {
            binary_path = Path::new("vb_server").to_path_buf();
        } else if os == "macos" {
            let current_exe = env::current_exe().expect("Failed to get current executable path");
            let bundle_path: PathBuf = current_exe
                .parent()
                .expect("Failed to get bundle path")
                .to_path_buf();
            binary_path = bundle_path.join("vb_server");
            println!("Bundle path: {}", bundle_path.display());
            println!("Binary path: {}", binary_path.display());
        } else if os == "windows" {
            binary_path = Path::new("vb_server.exe").to_path_buf();
            println!("Binary path: {:?}", binary_path);

            // Windows-specific process creation with CREATE_NO_WINDOW
            #[cfg(windows)]
            let child = Command::new(binary_path.as_path())
                .creation_flags(0x08000000) // CREATE_NO_WINDOW flag for Windows
                .spawn()
                .expect("Failed to start process");
            #[cfg(windows)]
            tauri::Builder::default()
                .on_window_event(move |event| match event.event() {
                    tauri::WindowEvent::Destroyed { .. } => {
                        match kill_process(&child.id().to_string()) {
                            Ok(_) => println!("Process killed successfully."),
                            Err(e) => eprintln!("Failed to kill process: {}", e),
                        }
                    }
                    _ => {}
                })
                .setup(|vocab_builder| {
                    info!("Setting up the app");
                    let window = vocab_builder.get_window("main").unwrap();
                    window.show().unwrap();
                    window
                        .set_size(Size::Logical(LogicalSize {
                            width: 1600.0,
                            height: 900.0,
                        }))
                        .unwrap();
                    info!("Window should be visible now");
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("Error while running tauri application");
        } else {
            binary_path = Path::new("").to_path_buf();
        }
    }

    if os != "windows" {
        let child = Command::new(binary_path.as_path())
            .spawn()
            .expect("Failed to start process");

        tauri::Builder::default()
            .on_window_event(move |event| match event.event() {
                tauri::WindowEvent::Destroyed { .. } => match kill_process(&child.id().to_string())
                {
                    Ok(_) => println!("Process killed successfully."),
                    Err(e) => eprintln!("Failed to kill process: {}", e),
                },
                _ => {}
            })
            .setup(|vocab_builder| {
                info!("Setting up the app");
                let window = vocab_builder.get_window("main").unwrap();
                window.show().unwrap();
                window
                    .set_size(Size::Logical(LogicalSize {
                        width: 1600.0,
                        height: 900.0,
                    }))
                    .unwrap();
                info!("Window should be visible now");
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Error while running tauri application");
    }
}
