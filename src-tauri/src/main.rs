// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger;
use log::info;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;
use tauri::WindowEvent;

#[cfg(windows)]
use std::os::windows::process::CommandExt; // Windows-specific

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
    env_logger::init(); // Initializes the logger
    info!("Logging initialized"); // Log a message for testing
    let os: &str = env::consts::OS;
    let current_dir = env::current_dir().unwrap();
    let binary_path: PathBuf;

    if cfg!(debug_assertions) {
        //temp point to windows only
        binary_path = current_dir.join("server.exe");
    } else {
        if os == "linux" {
            binary_path = Path::new("server").to_path_buf();
        } else if os == "macos" {
            let current_exe = env::current_exe().expect("Failed to get current executable path");

            let bundle_path: PathBuf = current_exe
                .parent()
                .expect("Failed to get bundle path")
                .to_path_buf();

            binary_path = bundle_path.join("server");
            println!("Bundle path: {}", bundle_path.display());
            println!("Binary path: {}", binary_path.display());
        } else if os == "windows" {
            binary_path = Path::new("server.exe").to_path_buf();
            println!("Binary path: {:?}", binary_path);

            // Windows-specific process creation with CREATE_NO_WINDOW
            #[cfg(windows)]
            match Command::new(binary_path.as_path())
                .creation_flags(0x08000000) // CREATE_NO_WINDOW flag for Windows
                .spawn()
            {
                Ok(child) => {
                    println!("Process started successfully");
                }
                Err(e) => {
                    eprintln!("Failed to start process: {}", e);
                }
            }

            return; // Exit early for Windows after process creation
        } else {
            binary_path = Path::new("").to_path_buf();
        }
    }

    println!("Binary path: {:?}", binary_path);

    #[cfg(unix)]
    let child = Command::new(binary_path.as_path())
        .spawn()
        .expect("Failed to start process");

    tauri::Builder::default()
        // .on_window_event(move |event| match event.event() {
        //     WindowEvent::Destroyed { .. } => match kill_process(&child.id().to_string()) {
        //         Ok(_) => println!("Process killed successfully."),
        //         Err(e) => eprintln!("Failed to kill process: {}", e),
        //     },
        //     _ => {}
        // })
        .setup(|app| {
            info!("Setting up the app");
            let window = app.get_window("main").unwrap();
            window.show().unwrap(); // Try explicitly showing the window
            info!("Window should be visible now");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
