mod node;
mod packet;

use node::{Actions, Node};
use packet::{write_to_stdout, Message, Packet};
use std::io::{self, BufRead};

fn main() {
    let mut node: Node = Node::default();

    for line in io::stdin().lock().lines() {
        let message: Message = serde_json::from_str(&line.unwrap()).unwrap();

        match message.body {
            Packet::Init { .. } => {
                let to_send = node.on_init(message).unwrap();
                write_to_stdout(to_send);
            }
            Packet::Echo { .. } => {
                let to_send = node.on_echo(message).unwrap();
                write_to_stdout(to_send);
            }
            _ => {}
        }
    }
}
