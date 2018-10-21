#![feature(custom_attribute)]

extern crate actix_web;
extern crate actix_helloworld;
extern crate config;
extern crate log;
extern crate simplelog;

#[macro_use]
extern crate lazy_static;

use actix_web::{http, server, App, HttpRequest};
use actix_web::middleware::Logger;

use actix_helloworld::settings::{Settings};
use log::{info};
use simplelog::*;

#[derive(Clone)]
struct GlobalSetting(Settings);

lazy_static! {
    static ref global_settings: GlobalSetting = {
        GlobalSetting(Settings::new().expect("fail create settings"))
    };
}

trait UseSetting {
    fn settings(&self) -> Settings;
}

struct SettingProvider;

impl UseSetting for SettingProvider {
    fn settings(&self) -> Settings {
        global_settings.0.clone()
    }
}

trait EchoServerNameService: UseSetting {
    fn echo(&self) {
        info!("server name -> {:?}", self.settings());
    }
}

struct EchoServerNameServiceImpl;

impl UseSetting for EchoServerNameServiceImpl {
    fn settings(&self) -> Settings {
        SettingProvider.settings()
    }
}

impl EchoServerNameService for EchoServerNameServiceImpl {}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap()
        ]
    ).unwrap();
    EchoServerNameServiceImpl.echo();

    server::new(
        || App::new()
            .middleware(Logger::default())
            .route("/health", http::Method::GET, |_: HttpRequest| "OK")

        )
        .workers(100)
        .backlog(100)
        .keep_alive(server::KeepAlive::Timeout(75))
        .bind("0.0.0.0:8080").unwrap()
        .run();

}