use tauri::{Menu, Submenu, MenuItem,command, window::WindowBuilder, Window, WindowUrl,RunEvent,GlobalShortcutManager};
use tauri::{
    api::dialog::ask, CustomMenuItem,  WindowEvent,
};

pub fn get_menus() -> Menu {
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
            .add_item(set)
    );
    let menus = Menu::new()
        .add_submenu(submenu_gear)
        .add_submenu(submenu_customer);
    return menus;
}