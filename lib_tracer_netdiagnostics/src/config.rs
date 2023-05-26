use netdiag::Protocol;

#[derive(Debug)]
pub struct TraceRouteConf {
    pub host: String,
    pub proto: Protocol,
    pub count: usize,
    pub limit: u8,
    pub delay: u64,
    pub expiry: u64,
}

impl TraceRouteConf {
    fn new(proto: String, port: u16) -> Self {
        let mut s = Self::default();
        let proto = match proto.to_uppercase().as_str() {
            "ICMP" => Protocol::ICMP,
            "TCP" if port > 0 => Protocol::TCP(port),
            "UDP" if port > 0 => Protocol::UDP(port),
            _ => Protocol::default(),
        };
        s.proto = proto;
        s
    }
}

impl Default for TraceRouteConf {
    fn default() -> Self {
        Self {
            host: String::default(),
            proto: Protocol::default(),
            count: 2,
            limit: 30,
            delay: 50,
            expiry: 250,
        }
    }
}
