use std::io::{self, Write};
use websocket::{client::builder::ClientBuilder, Message, message::Type};

fn main() {
    let addr = "ws://127.0.0.1:9095";

    let mut client = ClientBuilder::new(addr).unwrap();

    let mut conn = client.connect_insecure().unwrap();

    let msg = Message::from(conn.recv_message().unwrap());

    match msg.opcode
    {
        Type::Text => println!("Mensagem: {}", std::str::from_utf8(&msg.payload).unwrap()),
        _ => println!("Mensagem de tipo desconhecido.")
    }

    let mut input;

    let stdin = io::stdin();

    loop
    {
        input = "".to_string();

        print!("Mensagem para enviar ao servidor: ");
        _ = io::stdout().flush();
        _ = stdin.read_line(&mut input);

        let res = conn.send_message(&Message::text(input.clone()));

        if res.is_ok()
        {
            println!("Mensagem enviada com sucesso!");
        }
    }
}