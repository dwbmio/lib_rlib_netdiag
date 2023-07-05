#[derive(Debug)]
pub struct PingConf {
    pub count: usize,
    pub limit: u8,
    pub delay: u64,
    pub expiry: u64,
}

impl PingConf {
    pub fn new(_host: String, _proto: String, _port: u16) -> Self {
        let s = Self::default();
        // s.host = host;
        // s.proto = proto;
        s
    }
}

impl Default for PingConf {
    fn default() -> Self {
        Self {
            count: 2,
            limit: 30,
            delay: 50,
            expiry: 250,
        }
    }
}
