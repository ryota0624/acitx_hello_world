extern crate reqwest;

use settings::*;


pub trait Client: UseSetting {
    fn get_server_name(&self) -> Result<String, String> {
        let hello_server_setting = self.settings().hello_server;
        let server_url = format!("http://{}:{}/name", hello_server_setting.host, hello_server_setting.port);
        let client = reqwest::Client::new();
        let mut res = client
            .get(&server_url.to_string())
            .send()
            .expect("fail request");

        if res.status().is_success() {
            let response_text = res
                .text()
                .map_err(|error| format!("{:?}", error));
            response_text
        } else {
            Err("fail send request".to_string())
        }
    }
}

pub struct ClientImpl;

impl UseSetting for ClientImpl {
    fn settings(&self) -> Settings {
        SettingProvider.settings()
    }
}
impl Client for ClientImpl {}

pub trait UseClient {
    fn client(&self) -> Box<Client>;
}

pub trait ClientProvider {
}

impl UseClient for ClientProvider {
    fn client(&self) -> Box<Client> {
        Box::new(ClientImpl)
    }
}


