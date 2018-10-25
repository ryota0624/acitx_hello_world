extern crate actix_web;
extern crate actix;
extern crate actix_helloworld;
extern crate config;
extern crate log;
extern crate log4rs;
extern crate futures;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;


use actix_web::{http, server::HttpServer, App, HttpRequest, server, Json};
use actix_web::middleware::Logger;
use actix_helloworld::settings::Settings;
use log::*;
use std::thread;
use std::sync::{Arc};
use std::io;
use futures::future::Future;

#[derive(Clone)]
struct GlobalSetting(Settings);

lazy_static! {
    static ref global_settings: GlobalSetting = {
        GlobalSetting(Settings::new().expect("fail create settings"))
    };

    static ref global_counter: GlobalCounter = {
          GlobalCounter(Count(Arc::new(1)))
    };
}

#[derive(Clone)]
struct Count(Arc<i64>);

impl Count {
    fn int_value(self) -> Arc<i64> {
        self.0.clone()
    }
}

trait Counter {
    fn get_count(&self) -> Count;
}
#[derive(Clone)]
struct StateCounter(Count);

impl Counter for StateCounter {
    fn get_count(&self) -> Count {
      self.0.clone()
    }
}
#[derive(Clone)]
struct GlobalCounter(Count);

impl Counter for GlobalCounter {
    fn get_count(&self) -> Count {
        self.0.clone()
    }
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
    fn echo(&self) -> String {
        format!("{}", self.settings().server_name)
    }
}

struct EchoServerNameServiceImpl;

impl UseSetting for EchoServerNameServiceImpl {
    fn settings(&self) -> Settings {
        SettingProvider.settings()
    }
}

impl EchoServerNameService for EchoServerNameServiceImpl {}

#[derive(Serialize)]
struct ServerNameResponse {
    name: String,
}

#[derive(Serialize)]
struct CounterResponse {
    state_count: i64,
    global_count: i64,
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let setting = global_settings.0.clone();
    let server_setting = global_settings.0.server.clone();
    info!("server setting {:?}", global_settings.0);

    let sys = actix::System::new(setting.server_name);

    let state = StateCounter(Count(Arc::new(0)));
    let http_server_addr = HttpServer::new(
        move ||
            App::with_state(state.clone())
                .middleware(Logger::default())
                .route("/health", http::Method::GET, |_: HttpRequest<StateCounter>| "OK")
                .route("/count", http::Method::GET, |req: HttpRequest<StateCounter>| {

                    let state_count =
                        req.state().get_count().int_value();

                    let global_count =
                        global_counter.get_count().int_value();

                    Json(CounterResponse {
                        state_count,
                        global_count,
                    })
                })
                .route("/name", http::Method::GET, |_: HttpRequest<StateCounter>| Json(ServerNameResponse { name: EchoServerNameServiceImpl.echo() }))
    ).workers(server_setting.workers)
        .backlog(server_setting.backlog)
        .keep_alive(server::KeepAlive::Timeout(server_setting.timeout))
        .bind("0.0.0.0:8080")
        .unwrap().start();

    info!("start server application...");


    let _ = match global_settings.0.mode {
        actix_helloworld::settings::ServerMode::Dev => {
            debug!("application mode is Dev");
            Some(thread::spawn(move || {
                let mut input = String::new();
                loop {
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            if input == "exit\n" {
                                let _ = http_server_addr.send(server::StopServer { graceful: true }).wait();
                                std::process::exit(1);
                            }
                            input = "".to_string();
                        }
                        Err(error) => debug!("error: {}", error),
                    }
                }
            }))
        }
        _ => None
    };

    sys.run();
}