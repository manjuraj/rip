use std::net::{TcpListener, TcpStream};

pub struct Server {
    pub ip: String,
    pub port: i16,
    pub clients: Vec<TcpStream>
}

impl Server {
    pub fn new(ip: &str, port: &i16) -> Server {
        return Server {
            ip: ip.to_string(),
            port: *port,
            clients: Vec::new()
        }
    }

    pub fn handle_client(&mut self, stream: TcpStream) {
        self.clients.push(stream);
        println!("msg from client!");
    }

    pub fn start(&mut self) {
        let addr: String = format!("{}:{}", self.ip, self.port);
        let listener = TcpListener::bind(&*addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // connection succeeded
                    self.handle_client(stream)
                }
                Err(_) => { /* connection failed */ }
            }
        }
        // close the socket server
        drop(listener);
    }

}

pub fn new_server(ip: &str, port: &i16) -> Server {
    return Server::new(ip, port);
}
