use settings::*;

pub trait Client: UseSetting {
    fn get_server_name(&self) -> String {
        self.settings().hello_server_host
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


