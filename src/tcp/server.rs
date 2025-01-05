pub struct TcpServer {
    /// Server address
    address: Option<&'static str>,
    /// Port number
    port: i16,
}

impl TcpServer {
    fn new(address: &'static str, port: i16) -> Self {
        Self {
            address: Some(address),
            port,
        }
    }
}

impl Default for TcpServer {
    fn default() -> Self {
        Self::new("localhost", 8080)
    }
}
