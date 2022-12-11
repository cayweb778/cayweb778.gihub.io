use axum::Router;
use std::net::SocketAddr;
use reverse_terminal::register_terminal_router;

use tracing_subscriber::Layer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;


use std::io::Write;
use std::{env};
use std::borrow::Borrow;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::collections::HashMap;
use std::io::Read;
use printers::print;

use tauri::{AppHandle, Manager, Wry, api};

use tauri::{WindowMenuEvent,App,SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Menu, Submenu, CustomMenuItem, MenuItem, Window, WindowUrl};

mod short_car;
mod rest;
mod menue;

use menue::menu_hello::{title_menu,system_traymenu,system_tray_event,on_title_menu_event};
use rest::hello::{
    abchello,
    generate,
    close_app,
    go_appaaa,
    get_cache_dir,
    go_app,
    get_cache_ip_addr_api,
};


pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where S: tracing::Subscriber
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field={}", field.name());
        }

        let mut visitor = PrintlnVisitor;
        event.record(&mut visitor);
    }
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("  field={} value={:?}", field.name(), value)
    }
}

use std::thread;
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = tauri::Builder::default()
        .system_tray(system_traymenu())
        // 添加菜单
        .menu(title_menu())

        // 监听自定义菜单事件
        .on_menu_event(on_title_menu_event)
        .on_system_tray_event(system_tray_event)
        .setup(|app| {
            abchello(app);

            short_car::register_short(app);
            Ok(())
        })

        .invoke_handler(tauri::generate_handler![
            // clear_cache,
            generate,
            close_app,
            go_appaaa,
            go_app,get_cache_ip_addr_api,
            rest::print::get_printers_all,
            rest::print::print_data,get_cache_dir,
                hell
        ]);
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");


    // th2.join().unwrap();




    Ok(())
}
#[tauri::command]
async fn hell() {

    println!("asdasdassad{}","");
    tracing_subscriber::registry().with(CustomLayer).init();

    let apps66 = register_apps_router();

    let app2 = Router::new()
        .nest("/apps", apps66);

    let addr2 = SocketAddr::from(([127, 0, 0, 1], 9099));
    tracing::debug!("listening on {}", addr2);
    axum::Server::bind(&addr2)
        .serve(app2.into_make_service())
        .await
        .unwrap();
    println!("hasdasdasdasdsada{}","");
}
fn register_apps_router() -> Router {
    register_terminal_router(Router::new())
}