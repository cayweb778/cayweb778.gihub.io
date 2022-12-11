#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::io::Write;
use std::{env};
use std::borrow::Borrow;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::collections::HashMap;
use std::io::Read;

use tauri::{App,AppHandle, Manager, Wry, api};

use tauri::{SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Menu, Submenu, CustomMenuItem, MenuItem, Window, WindowUrl};


fn has_filename_exist(path: &str, filename: &str) -> bool {
    let dir = std::fs::read_dir(path).unwrap();
    for x in dir {
        if let Ok(path) = x {
            // 是否存在某个文件
            if path.file_name().eq(filename) {
                return true;
            }
        }
    }
    return false;
}

fn get_txt(path: &str, filename: &str) -> String {
    let filepath = format!("{}/{}", path, filename);
    let mut file = std::fs::File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.to_string();
}


fn usepath_info() -> HashMap<String, String> {
    let ddd = api::path::data_dir().unwrap();
    let ccc = ddd.as_path().to_str().unwrap();
    let mut scores = HashMap::new();
    scores.insert(String::from("dirpath"), String::from(ccc.to_string()));
    scores.insert(String::from("filename"), String::from("NcServerAddr.txt"));
    return scores;
}

fn get_cache_ip_addr() -> HashMap<String, String> {
    let path_info = usepath_info();
    let dirpath = path_info.get("dirpath").unwrap();
    let filename = path_info.get("filename").unwrap();
    let is_exist = has_filename_exist(&dirpath, &filename);

    let mut scores = HashMap::new();
    if is_exist {
        scores.insert(String::from("code"), String::from("200"));
        scores.insert(String::from("msg"), String::from("成功获取"));
        scores.insert(String::from("data"), String::from(get_txt(&dirpath, filename)));
    } else {
        scores.insert(String::from("code"), String::from("404"));
        scores.insert(String::from("msg"), String::from("找不到"));
        scores.insert(String::from("data"), String::from(""));
    }
    // let a=serde_json::to_string(&scores).unwrap();
    return scores;
}


#[tauri::command]
pub fn get_cache_ip_addr_api() -> String {
    let scores = get_cache_ip_addr();
    return serde_json::to_string(&scores).unwrap();
}


#[tauri::command]
pub fn generate(name: String) -> String {
    let path_info = usepath_info();
    let dirpath = path_info.get("dirpath").unwrap();
    let filename = path_info.get("filename").unwrap();
    let mut file = std::fs::File::create(format!("{}/{}", dirpath, filename)).expect("create failed");
    file.write_all(name.as_bytes()).expect("write failed");
    return format!("{}::{}::{}", dirpath, filename, dirpath);
}

#[tauri::command]
pub fn go_app(_name: String, window: Window) -> String {
    go_app2(window.app_handle().borrow());
    return "".to_string();
    // let path_info=usepath_info();
    // let dirpath =path_info.get("dirpath").unwrap();
    // let filename=path_info.get("filename").unwrap();
    // println!("无敌：{}",dirpath);
    // println!("无敌：{}",filename);
    // let mut file = std::fs::File::create(format!("{}{}",dirpath,filename)).expect("create failed");
    // file.write_all(name.as_bytes()).expect("write failed");
    // return format!("{}::{}::{}",dirpath,filename,dirpath)
}


#[tauri::command]
pub fn close_app(window: Window) -> String {
    let app = window.app_handle();
    let aa = app.windows().keys();
    // println!("{:?}",app.windows().keys());
    // println!("{:?}","sdasdas");
    app.get_window("main_dy").unwrap().close().expect("dsadsad");
    return "".to_string();
    // let path_info=usepath_info();
    // let dirpath =path_info.get("dirpath").unwrap();
    // let filename=path_info.get("filename").unwrap();
    // println!("无敌：{}",dirpath);
    // println!("无敌：{}",filename);
    // let mut file = std::fs::File::create(format!("{}{}",dirpath,filename)).expect("create failed");
    // file.write_all(name.as_bytes()).expect("write failed");
    // return format!("{}::{}::{}",dirpath,filename,dirpath)
}


// register shortcut

fn go_app2(app: &AppHandle<Wry>) {
    let cache_ip_addr_map = get_cache_ip_addr();
    let url = cache_ip_addr_map["data"].clone();


    let window_url = format!("http://{}/nc/", url);
    let name = "main_dy";
    // if app.get_window("main_dy1").is_none() {
    //     if !app.get_window("main_dy2").is_none() {
    //         app.get_window("main_dy2").unwrap().close().expect("dsadsad");
    //     }
    //     name = "main_dy1";
    // } else {
    //     name = "main_dy2";
    //     app.get_window("main_dy1").unwrap().close().expect("sadsadas");
    // }
    let _win = tauri::window::WindowBuilder::new(app, name.to_string(), WindowUrl::App(window_url.into()))
        .title("财税达ERP-NC企业管理软件")
        .maximized(true)
        .inner_size(600.0, 400.0)
        .build().unwrap();
}

// #[tauri::command]
// fn clear_cache(window: Window) -> &'static str {
//     window.close().expect("sdasdas");
//     if !window.get_window("main_dy2").is_none(){
//
//         window.get_window("main_dy2").unwrap().close().expect("hesa");
//     }else{
//
//         window.get_window("main_dy1").unwrap().close().expect("hesa");
//     }
//     let aaaa=api::path::local_data_dir();
//     let ccccc=format!("{}/org.boozsoft.ncapp",aaaa.unwrap().as_path().to_str().unwrap().to_string());
//     println!("{}",ccccc);
//     go_app2(window.app_handle().borrow());
//     return "hello";
// }
#[tauri::command]
pub fn get_cache_dir() {}


fn go_app2a(app: &AppHandle<Wry>) {}

#[tauri::command]
pub async fn go_appaaa(_name: String, handle: tauri::AppHandle) {
    let cache_ip_addr_map = get_cache_ip_addr();
    let url = cache_ip_addr_map["data"].clone();


    let window_url = format!("http://{}/nc/", url);
    let name = "main_dy";
    // if app.get_window("main_dy1").is_none() {
    //     if !app.get_window("main_dy2").is_none() {
    //         app.get_window("main_dy2").unwrap().close().expect("dsadsad");
    //     }
    //     name = "main_dy1";
    // } else {
    //     name = "main_dy2";
    //     app.get_window("main_dy1").unwrap().close().expect("sadsadas");
    // }
    // let hello = &aa.get_window("config").unwrap().hwnd().unwrap();
    let _win = tauri::window::WindowBuilder::new(&handle, name.to_string(), WindowUrl::App(window_url.into()))
        .title("财税达ERP-NC企业管理软件")
        .maximized(true)
        .inner_size(600.0, 400.0)
        .build().unwrap();
    //
    //
    // let _win = tauri::window::WindowBuilder::new(&aa, name.to_string(), WindowUrl::App(window_url.into()))
    //     .title("财税达ERP-NC企业管理软件2222222")
    //     .maximized(true)
    //     .parent_window(*hello)
    //     .inner_size(600.0, 400.0)
    //     .build().unwrap();
    // _win.open_devtools();
}

pub fn abchello(app: &App<Wry>){
    let _win = tauri::window::WindowBuilder::new(app, "config".to_string(), WindowUrl::App("config".into()))
        .title("财税达配置选项")
        .inner_size(600.0, 400.0)
        .center()
        .always_on_top(true)

        .decorations(false)
        .build().unwrap();
    // #[cfg(any(debug_assertions, feature = "devtools"))]
    // _win.open_devtools();
}
