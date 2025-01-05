use tokio::net::{TcpListener,  TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct HttpServer {
    /// Root path containing all of the files and directories which the server exposes.
    _root: &'static str,
    address: &'static str,
    port: i16,
}

impl HttpServer {
    pub fn new(address: &'static str, port: i16, root: &'static str) -> Self {
        Self {
            address,
            port,
            _root: root,
        }
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port)).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            self.handle(socket).await;
        }
    }

    async fn handle(&self, mut socket: TcpStream) {
        tokio::spawn(async move {
            let mut buf = [0; 1024*16];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("closed connection");
                        return
                    },
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from tcp socket: {:?}", e);
                        return;
                    }
                };

                println!("Read {} bytes", n);

                // ping pong
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to tcp socket: {:?}", e);
                    return;
                }
            }
        });
    }
}
