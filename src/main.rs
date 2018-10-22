extern crate actix_web;
extern crate actix_helloworld_client;
extern crate config;
extern crate log;
extern crate log4rs;

use actix_web::{http, server, App, HttpRequest};
use actix_web::middleware::Logger;

use actix_helloworld_client::settings::*;
use actix_helloworld_client::{hello_client};
use actix_helloworld_client::hello_client::{UseClient, Client};

use log::info;

struct ClientHostService;

impl hello_client::ClientProvider for ClientHostService {
}

impl hello_client::UseClient for ClientHostService {
    fn client(&self) -> Box<Client> {
        (self as &hello_client::ClientProvider).client()
    }
}


impl ClientHostService {
    fn run(&self) -> String {
        self.client().get_server_name()
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let server_setting = SettingProvider.settings().server;
    info!("setting {:?}",  SettingProvider.settings());

    let app_server = server::new(
        || App::new()
            .middleware(Logger::default())
            .route("/health", http::Method::GET, |_: HttpRequest| "OK")
            .route("/server_host", http::Method::GET, |_: HttpRequest|
                ClientHostService.run() )

    )
        .workers(server_setting.workers)
        .backlog(server_setting.backlog)
        .keep_alive(server::KeepAlive::Timeout(server_setting.timeout))
        .bind("0.0.0.0:8080")
        .unwrap();

    info!("start server application...");

    app_server.run();
}