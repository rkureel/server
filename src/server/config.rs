pub struct ServerConfig {
    address: String,
    port: u32,
}

impl ServerConfig {
    pub fn new(address: &str, port: u32) -> Self {
        return ServerConfig { 
            address: String::from(address), 
            port: port
        } 
    }

    pub fn get_address(&self) -> String {
        return format!("{}:{}", self.address, self.port);
    }
}

