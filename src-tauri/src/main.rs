// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fix_path_env::fix;
use std::env;
use std::process::Command;
use std::path::PathBuf;
use std::path::Path;
use tauri::WindowEvent;

fn kill_process(pid: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        println!("Sending INT signal to process with PID: {}", pid);

        let mut kill = Command::new("kill")
            .args(["-s", "SIGINT", &pid])
            .spawn()?;
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
  // read from .env
  dotenv::load().ok();

  // if cfg!(debug_assertions) {
  //   dotenv::from_filename(".env.development").unwrap().load();
  // } else {
  //   dotenv::from_filename(".env").unwrap().load();
  // }

  if let Err(e) = fix() {
    println!("{}", e);
  } else {
        let current_dir = env::current_dir().unwrap();
        let binary_path: PathBuf;
        if cfg!(debug_assertions) {
          binary_path = current_dir.join("binaries/server-x86_64-unknown-linux-gnu");
        } else {
          //This works for Linux AppImage. Need just server for MacOS App bundle
          // binary_path = current_dir.join("bin/server");//
          binary_path = Path::new("server").to_path_buf();
        }
        println!("Binary path: {:?}", binary_path);

        let child = Command::new(binary_path.as_path())
            .spawn()
            .expect("Failed to start process");

        tauri::Builder::default()
            .on_window_event(move |event| match event.event() {
                WindowEvent::Destroyed { .. } => {
                    match kill_process(&child.id().to_string()) {
                        Ok(_) => println!("Process killed successfully."),
                        Err(e) => eprintln!("Failed to kill process: {}", e),
                    }
                }
                _ => {}
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
  }
}