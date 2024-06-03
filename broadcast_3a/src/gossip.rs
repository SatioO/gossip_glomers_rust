use crate::{
    node::Node,
    packet::{Message, RequestBody},
};

pub trait Gossip {
    fn on_topology(&mut self, message: Message) -> Option<Message>;
    fn on_read(&mut self, message: Message) -> Option<Message>;
    fn on_broadcast(&mut self, message: Message) -> Option<Message>;
    fn on_error(&mut self, message: Message) -> Option<Message>;
}

impl Gossip for Node {
    fn on_topology(&mut self, message: Message) -> Option<Message> {
        match message.body {
            RequestBody::Topology {
                msg_id, topology, ..
            } => {
                self.storage.init_topology(topology);

                let to_send = Message {
                    src: self.id.clone(),
                    dest: message.src,
                    body: RequestBody::TopologyOk {
                        msg_id,
                        in_reply_to: msg_id,
                    },
                };

                Some(to_send)
            }
            _ => None,
        }
    }

    fn on_read(&mut self, message: Message) -> Option<Message> {
        match message.body {
            RequestBody::Read { msg_id } => {
                let to_send = Message {
                    src: self.id.clone(),
                    dest: message.src,
                    body: RequestBody::ReadOk {
                        msg_id,
                        in_reply_to: msg_id,
                        messages: self.storage.get_messages(),
                    },
                };

                Some(to_send)
            }
            _ => None,
        }
    }

    fn on_broadcast(&mut self, message: Message) -> Option<Message> {
        match message.body {
            RequestBody::Broadcast {
                msg_id,
                message: msg,
            } => {
                self.storage.add_message(msg);

                let to_send = Message {
                    src: self.id.clone(),
                    dest: message.src,
                    body: RequestBody::BroadcastOk {
                        msg_id,
                        in_reply_to: msg_id,
                    },
                };

                Some(to_send)
            }
            _ => None,
        }
    }

    fn on_error(&mut self, message: Message) -> Option<Message> {
        match message.body {
            RequestBody::Error {
                text,
                code,
                in_reply_to,
            } => {
                let to_send = Message {
                    src: self.id.clone(),
                    dest: message.src,
                    body: RequestBody::Error {
                        in_reply_to,
                        code,
                        text,
                    },
                };

                Some(to_send)
            }
            _ => None,
        }
    }
}
