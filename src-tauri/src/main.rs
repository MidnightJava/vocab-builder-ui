// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fix_path_env::fix;
use std::process::{Command};
use tauri::{WindowEvent};

static PROGRAM:&str = "binaries/server-x86_64-unknown-linux-gnu";

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
  if let Err(e) = fix() {
    println!("{}", e);
  } else {
    let child = Command::new(&PROGRAM)
        .spawn()
        .expect("Failed to start process");
    tauri::Builder::default()
    .on_window_event(move|event| match event.event() {
        WindowEvent::Destroyed {  .. } => {
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
