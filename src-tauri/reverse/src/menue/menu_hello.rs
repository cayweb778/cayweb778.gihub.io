use std::collections::HashMap;
use std::io::Read;

use tauri::{AppHandle, Manager, Wry, api};

use tauri::{WindowMenuEvent,SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Menu, Submenu, CustomMenuItem, MenuItem, Window, WindowUrl};

// 应用菜单处理事件
pub fn on_title_menu_event(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "close" => {
            event.window().close().unwrap();
        },
        "new_file" => {
            // debug 信息（终端输出）
            dbg!("new file");
        }
        "edit_file" => {
            // 发送信息到菜单所属窗口（弹窗形式）
            // message(win, "Eidt File", "TODO");
        }
        "undo" => {
            dbg!("undo");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => {}
    }
}


pub fn system_tray_event(app: &AppHandle<Wry>, e: SystemTrayEvent) { //托盘事件
    match e {
        SystemTrayEvent::LeftClick { //左键事件
            position: _,
            size: _,
            ..
        } => { //显示窗口
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() { //托盘菜单点击事件
            "quit" => { //退出程序
                std::process::exit(0);
            }
            "hide" => { //隐藏窗口
                let window = app.get_window("main").unwrap(); //获取窗口实例
                window.hide().unwrap();
            }
            "show" => { //显示窗口并聚焦
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn title_menu() -> Menu {
    let submenu_gear = Submenu::new(
        "Gear",
        Menu::new()
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Zoom)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );
    let close = CustomMenuItem::new("close".to_string(), "关闭");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let set = CustomMenuItem::new("set".to_string(), "设置");
    let submenu_customer = Submenu::new(
        "财税达菜单",
        Menu::new()
            .add_item(close)
            .add_item(quit)
            .add_item(set),
    );
    let menus222 = Menu::new()
        .add_submenu(submenu_gear)
        .add_submenu(submenu_customer);
    return menus222;
}

// 托盘菜单
pub fn system_traymenu() -> SystemTray {
    // let a= [68, 114, 105, 118, 101, 114, 78, 97, 109, 101, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 78, 97, 109, 101, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 77, 83, 45, 88, 80, 83, 32, 67, 108, 97, 115, 115, 32, 68, 114, 105, 118, 101, 114, 32, 50, 32, 32, 32, 32, 72, 101, 108, 108, 111, 66, 49, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 77, 83, 45, 88, 80, 83, 32, 67, 108, 97, 115, 115, 32, 68, 114, 105, 118, 101, 114, 32, 50, 32, 32, 32, 32, 72, 101, 108, 108, 111, 65, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 73, 80, 80, 32, 67, 108, 97, 115, 115, 32, 68, 114, 105, 118, 101, 114, 32, 32, 32, 32, 32, 32, 32, 32, 32, 80, 68, 70, 119, 114, 105, 116, 101, 114, 32, 64, 32, 99, 97, 121, 161, 175, 115, 32, 77, 97, 99, 66, 111, 111, 107, 32, 65, 105, 114, 32, 32, 13, 13, 10, 71, 101, 110, 101, 114, 105, 99, 32, 47, 32, 84, 101, 120, 116, 32, 79, 110, 108, 121, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 65, 98, 99, 49, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 71, 101, 110, 101, 114, 105, 99, 32, 47, 32, 84, 101, 120, 116, 32, 79, 110, 108, 121, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 87, 117, 106, 105, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 83, 111, 102, 116, 119, 97, 114, 101, 32, 80, 114, 105, 110, 116, 101, 114, 32, 68, 114, 105, 118, 101, 114, 32, 32, 79, 110, 101, 78, 111, 116, 101, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 88, 80, 83, 32, 68, 111, 99, 117, 109, 101, 110, 116, 32, 87, 114, 105, 116, 101, 114, 32, 118, 52, 32, 32, 32, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 88, 80, 83, 32, 68, 111, 99, 117, 109, 101, 110, 116, 32, 87, 114, 105, 116, 101, 114, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 80, 114, 105, 110, 116, 32, 84, 111, 32, 80, 68, 70, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 80, 114, 105, 110, 116, 32, 116, 111, 32, 80, 68, 70, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 71, 101, 110, 101, 114, 105, 99, 32, 47, 32, 84, 101, 120, 116, 32, 79, 110, 108, 121, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 71, 101, 110, 101, 114, 105, 99, 32, 47, 32, 84, 101, 120, 116, 32, 79, 110, 108, 121, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 77, 105, 99, 114, 111, 115, 111, 102, 116, 32, 83, 104, 97, 114, 101, 100, 32, 70, 97, 120, 32, 68, 114, 105, 118, 101, 114, 32, 32, 32, 32, 32, 32, 32, 32, 70, 97, 120, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 13, 13, 10, 13, 13, 10];
    // let mut hello = String::new();
    // gb2312::decode(&a,& mut hello);
    // // let cc=a.to_vec();
    // // let ddd=String::from_utf8(cc);
    // // println!("{}",ddd.unwrap());
    // println!("{}",hello);
    let quit = CustomMenuItem::new("quit".to_string(), "关闭窗口");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let add = CustomMenuItem::new("add".to_string(), "打印机队列");
    let add1 = CustomMenuItem::new("add1".to_string(), "财税达控制中心");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_item(add)
        .add_item(add1)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    return system_tray;
}