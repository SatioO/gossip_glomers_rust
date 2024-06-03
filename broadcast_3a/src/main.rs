mod gossip;
mod node;
mod packet;

use gossip::Gossip;
use node::Node;
use packet::{write_to_stdout, Message, RequestBody};
use std::io::{stdin, BufRead};

fn main() {
    let mut node = init_node();

    for line in stdin().lock().lines() {
        let message: Message = Message::parse_message(line.unwrap());

        match message.body {
            RequestBody::Topology { .. } => {
                let to_send = node.on_topology(message).unwrap();
                write_to_stdout(to_send);
            }
            RequestBody::Read { .. } => {
                let to_send = node.on_read(message).unwrap();
                write_to_stdout(to_send);
            }
            RequestBody::Broadcast { .. } => {
                let to_send = node.on_broadcast(message).unwrap();
                write_to_stdout(to_send);
            }
            RequestBody::Error { .. } => {
                let to_send = node.on_error(message).unwrap();
                write_to_stdout(to_send);
            }
            _ => panic!("unknown type"),
        };
    }
}

fn init_node() -> Node {
    let mut message = String::new();
    stdin().read_line(&mut message).unwrap();

    let init_message = Message::parse_message(message);
    let node;
    match init_message.body {
        RequestBody::Init { msg_id, .. } => {
            node = Node::init(init_message.clone());

            let to_send = Message {
                src: init_message.dest,
                dest: init_message.src,
                body: RequestBody::InitOk {
                    in_reply_to: msg_id,
                },
            };

            write_to_stdout(to_send);
        }
        _ => panic!("Node is not initialized yet"),
    };

    node
}
