use std::borrow::Borrow;
use printers::{get_printers};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::process::Command;


fn window_get_all() -> String {
    println!("{}dsadasda","");
    let out=Command::new("wmic").arg("printer").arg("get").arg("Name").output();
    let ccc;
    if out.is_err() {
        ccc= Result::Err(out.unwrap_err().to_string());
    }else {
        let mut hello = String::new();
        let abc=out.unwrap().stdout;
        textcode::gb2312::decode(&abc,& mut hello);
        ccc=Result::Ok(hello);
    }
    let bbb=ccc.unwrap();

    let mut lines: Vec<&str> = bbb.split_inclusive("\n").collect();
    lines.remove(0);


    let mut new_arr: Vec<HashMap<String, String>> = Vec::new();
    // let mut printers = Vec::with_capacity(lines.len());

    for line in lines {
        let mut scores = HashMap::new();
        // let printer_data: Vec<&str> = line.split_ascii_whitespace().collect();

        // let name = String::from(printer_data[1]);
        // let system_name = String::from(printer_data[0]);
        scores.insert("id".to_string(), line.to_string());
        scores.insert("name".to_string(), line.to_string());
        scores.insert("system_name".to_string(), line.to_string());
        new_arr.push(scores);
        // printers.push(printer::Printer::new(name, system_name,  &self::print));

    }
    return serde_json::to_string(&new_arr).unwrap();
}

#[tauri::command]
pub fn get_printers_all() -> String {
    if cfg!(windows) {
        return window_get_all();
    }

    if cfg!(unix) {
        let printers = get_printers();
        let mut new_arr: Vec<HashMap<String, String>> = Vec::new();
        for (index, value) in printers.iter().enumerate() {
            let mut scores = HashMap::new();
            scores.insert("id".to_string(), value.id.to_string());
            scores.insert("name".to_string(), value.name.to_string());
            scores.insert("system_name".to_string(), value.system_name.to_string());
            new_arr.push(scores);
        }
        return serde_json::to_string(&new_arr).unwrap();
    }
    panic!("Unsupported OS");






}
use base64::{ decode as base64Decode};
use tauri::{api, Manager, Window};

use uuid::Uuid;
use std::io::{Read, Write};
use anyhow::Error;

fn abc111(path: String, aaa: &[u8]) -> Result<(), Error> {
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(aaa).expect("write failed");
    file.flush().unwrap();
    Ok(())
}
use std::thread::sleep;
use std::time::Duration;

#[tauri::command]
pub async fn print_data(printer_name:String, contents:String, window:Window) {
    let p1=printer_name.clone();
    let p2=printer_name.clone();
    if cfg!(windows) {
        let abc = window.app_handle();
        let resource_dir = abc.path_resolver().resource_dir().unwrap();
        let exe_path = format!("{}/hello/SumatraPDF-3.4.6-64.exe", resource_dir.to_str().unwrap());

        let cache_dir_fn = abc.path_resolver().app_cache_dir().unwrap();
        let cache_dir = cache_dir_fn.to_str().unwrap();

        // write file
        let base64_decode_vec = base64Decode(contents.clone()).unwrap();
        let aaa = &base64_decode_vec[..];
        let id = Uuid::new_v4();
        let filename = id.to_string();
        abc111(format!("{}/{}", cache_dir, filename), aaa).unwrap();
        sleep(Duration::from_secs(2));
        let aaa = printer_name.clone();
        // start print
        let file_name2 = format!("{}/{}", cache_dir, filename);
        let printer_target = printer_name;
        let output = Command::new(exe_path.clone().replace("\\\\?\\", ""))
            .args([
                "-print-to",
                &aaa.replace("\n", "").trim(),
                "-print-settings",
                "noscale",
                &file_name2,
            ])
            .output();
        println!("{}2222", "a");
        println!("{}:::", format!("{}/{}", cache_dir, filename));
        println!("{}:::", exe_path);
    }

    if cfg!(unix) {
        let printers = get_printers();
        let base64_decode_vec = base64Decode(contents.clone()).unwrap();
        for (index, value) in printers.iter().enumerate() {
            if (value.name.as_str() == &p2) {
                printers::print(value, &base64_decode_vec);
            }
            // let mut scores = HashMap::new();
            // scores.insert("id".to_string(), value.id.to_string());
            // scores.insert("name".to_string(), value.name.to_string());
            // scores.insert("system_name".to_string(), value.system_name.to_string());
            // new_arr.push(scores);
        }
    }
    panic!("Unsupported OS");
}