use std::net::TcpStream;
use websocket::client::sync::Client;
use websocket::message::Type;
use websocket::{sync::Server, Message};

fn main()
{
    let mut server = Server::bind("127.0.0.1:9095").unwrap();

    loop
    {
        match server.accept()
        {
            Ok(wsupgrade) => {
                std::thread::spawn(|| {
                    let mut client: Client<TcpStream> = wsupgrade.accept().unwrap();

                    let msg = Message::text("Hello, Client!");

                    _ = client.send_message(&msg);

                    loop
                    {
                        let msg = Message::from(client.recv_message().unwrap());

                        match msg.opcode
                        {
                            Type::Text => {
                                println!("Message received from client: {}", std::str::from_utf8(&msg.payload).unwrap());
                            }
                            _ => { }
                        }
                    }
                });
            }
            _ => { }
        }
    }
}
