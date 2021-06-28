pub struct Url {
    pub protocol: Protocol,
    pub host: String,
    pub path: String
}

impl Url {
    fn get(&self) -> String {
        format!("{}//{}/{}", self.protocol.get(), self.host, self.path)
    }
}

pub enum Protocol {
    Http,
    Https
}

impl Protocol {
    fn get(&self) -> &str {
        match self {
            Protocol::Http => "http",
            Protocol::Https => "https"
        }
    }
}
