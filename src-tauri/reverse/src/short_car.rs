use std::borrow::Borrow;
use tauri::{WindowUrl,  GlobalShortcutManager, Wry, Manager};
use tauri::{
    App,

};

pub fn register_short(app:&App<Wry>){

    let app_handle_clone=app.handle().clone();
    app.global_shortcut_manager().register("Shift+Escape", move|| {

        let window1 = app_handle_clone.borrow().get_window("config");
        // if window1.is_none(){
        //     let _win = tauri::window::WindowBuilder::new(app_handle_clone.borrow(), "config".to_string(), WindowUrl::App("config".into()))
        //         .title("财税达配置选项")
        //         .inner_size(600.0, 400.0)
        //         .center()
        //         .always_on_top(true)
        //
        //         .decorations(false)
        //         .build().unwrap();
        //     // #[cfg(any(debug_assertions, feature = "devtools"))]
        //     _win.open_devtools()
        // }else{
            if window1.clone().unwrap().is_visible().unwrap(){

                window1.clone().unwrap().hide().expect("TODO: panic message");
            }else{
                window1.clone().unwrap().show().expect("TODO: panic message");
            }
        // }


    }).expect("TODO: panic message");

}
