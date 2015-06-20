use self::networking::Server;

pub mod networking;

fn main() {
    let port: i16 = 9999;
    let ip = "localhost";
    let mut server = Server::new(&ip, &port);
    println!("starting rip on {}", port);
    server.start();
}
