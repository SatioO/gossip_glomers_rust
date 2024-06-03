use crate::packet::{Message, Packet};

#[derive(Default)]
pub struct Node {
    pub messages: Vec<u64>,
    pub node_id: String,
    pub message_id: u64,
}

pub trait Actions {
    fn on_init(&mut self, message: Message) -> Option<Message>;
    fn on_echo(&mut self, message: Message) -> Option<Message>;
}

impl Actions for Node {
    fn on_init(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Packet::Init {
                msg_id, node_id, ..
            } => {
                if !self.node_id.is_empty() {
                    self.node_id = String::from(node_id);
                }

                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Packet::InitOk {
                        in_reply_to: msg_id,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }

    fn on_echo(&mut self, message: Message) -> Option<Message> {
        match message.body {
            Packet::Echo { msg_id, echo } => {
                let to_send = Message {
                    src: message.dest,
                    dest: message.src,
                    body: Packet::EchoOk {
                        msg_id,
                        in_reply_to: msg_id,
                        echo,
                    },
                };

                return Some(to_send);
            }
            _ => None,
        }
    }
}
