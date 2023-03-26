#![windows_subsystem = "windows"]




#[cfg(windows)]
extern crate winapi;


use std::fmt::format;
use std::fs::*;
use std::io::*;

use chrono::{DateTime, Timelike, Utc};



//#[cfg(windows)]
fn log_header(file: &mut File) {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::sysinfoapi::{SYSTEM_INFO, GetSystemInfo};
    use winapi::um::winbase::{GetComputerNameW, GetVersionExW, GetDiskFreeSpaceExW};
    use winapi::um::winnt::{LANGIDFROMLCID, LCTYPE, LGRPIDFROMLCID, PROCESSOR_ARCHITECTURE_AMD64};
    use winapi::um::winnls::{GetKeyboardLayout, GetSystemDefaultLCID, GetLocaleInfoEx};
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::fileapi::GetLogicalDriveStringsW;
    use winapi::um::winbase::GetDiskFreeSpaceExW;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    let time: DateTime<Utc> = Utc::now();


    // Get system architecture
    let mut system_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
    unsafe {
        GetSystemInfo(&mut system_info);
    }
    let architecture = if system_info.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64 {
        "64-bit"
    } else {
        "32-bit"
    };
    let arch = format!("Architecture: {}", architecture);

    // Get Windows version
    let mut os_version_info: winapi::um::winnt::OSVERSIONINFOW = unsafe { std::mem::zeroed() };
    os_version_info.dwOSVersionInfoSize = std::mem::size_of::<winapi::um::winnt::OSVERSIONINFOW>() as DWORD;
    unsafe {
        GetVersionExW(&mut os_version_info);
    }
    let windows_version = format!("{}.{}", os_version_info.dwMajorVersion, os_version_info.dwMinorVersion);
    let winV = format!("Windows version: {}", windows_version);

    // Get keyboard layout
    let lcid = unsafe { GetKeyboardLayout(0) };
    let lang_id = LANGIDFROMLCID(lcid);
    let lgrpid = LGRPIDFROMLCID(lcid);
    let mut layout_buffer = [0u16; 9];
    let layout_length = unsafe {
        GetLocaleInfoEx(
            lcid,
            LCTYPE::LOCALE_SKEYBOARDSTOINSTALL,
            layout_buffer.as_mut_ptr() as _,
            layout_buffer.len() as i32,
        )
    };
    let keyboard_layout = if layout_length > 0 {
        let layout_string = OsString::from_wide(&layout_buffer[..layout_length as usize]);
        layout_string.to_string_lossy().into_owned()
    } else {
        String::from("Unknown")
    };
    let keyLayout = format!("Keyboard layout: {}", keyboard_layout);

    // Get system language
    let lcid = unsafe { GetSystemDefaultLCID() };
    let mut buffer = [0u16; 9];
    let length = unsafe {
        GetLocaleInfoEx(
            lcid,
            LCTYPE::LOCALE_SISO639LANGNAME,
            buffer.as_mut_ptr() as _,
            buffer.len() as i32,
        )
    };
    let language_code = if length > 0 {
        let lang_string = OsString::from_wide(&buffer[..length as usize]);
        lang_string.to_string_lossy().into_owned()
    } else {
        String::from("Unknown")
    };
    let sysLang = format!("System language: {}", language_code);

    // Get available disk space on all disks
    let mut storage = String::new();
    let mut buffer = [0u16; 261];
    let result = unsafe {
        GetLogicalDriveStringsW(
            buffer.len() as u32,
            buffer.as_mut_ptr(),
        )
    };
    if result > 0 {
        let drive_strings = OsString::from_wide(&buffer[..result as usize]);
        for drive_string in drive_strings.split_null_terminated() {
            let path = drive_string.to_string_lossy();
            let mut free_bytes = 0u64;
            let result = unsafe {
                GetDiskFreeSpaceExW(
                    path.encode_wide().collect::<Vec<_>>().as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut free_bytes as *mut u64,
                )
            };
            if result != 0 {
                storage = format!("Available storage on {}: {} bytes", path, free_bytes);
            }
        }
    }





    let write: String = format!("date: {} current time = {:02}+{:02}+{:02}\nOS info:\n{}\n{}\n{}\n{}\n{}", time.date_naive(), time.hour(), time.minute(), time.second(),arch, winV, keyLayout, sysLang, storage);
    file.write(write.as_bytes()).expect("help i am broken");
    file.write(b"--- Start of Logging ---").expect("TODO: panic message");
}


#[cfg(not(windows))]


fn log_write(file: &mut File, s: String){

}




#[cfg(not(windows))]
fn run(file: &mut File) {
    println!("hi");
    log_header(file);



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
    let filename = format!("log-{}-{:02}+{:02}+{:02}.log", now.date_naive(), now.hour(), now.minute(), now.second());

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













