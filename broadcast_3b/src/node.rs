// // pub enum Event<Payload> {
// //     Message(Message<Payload>),
// // }

// struct Body<Payload> {
//     id: Option<usize>,
//     in_reply_to: Option<usize>,
//     payload: Payload,
// }

// struct Message<Payload> {
//     src: String,
//     dest: String,
//     body: Body<Payload>,
// }

// trait Node<Payload> {
//     fn step(&mut self, input: Message<Payload>);
// }

// struct EchoNode {
//     id: usize,
// }

// enum Payload {
//     Echo { echo: String },
// }

// impl Node<String> for EchoNode {
//     fn step(&mut self, input: Message<String>) {
//         // match input.body.payload {

//         // }
//     }
// }

use serde::{Deserialize, Serialize};

use crate::{Body, Message};

pub trait Node<Payload> {
    fn step(&self, s: Message<Payload>) -> Message<Payload>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Init {},
    InitOk {},
}

pub struct BroadcastNode;

impl Node<BroadcastPayload> for BroadcastNode {
    fn step(&self, input: Message<BroadcastPayload>) -> Message<BroadcastPayload> {
        match input.body.payload {
            BroadcastPayload::Init {} => Message {
                src: input.dest,
                dest: input.src,
                body: Body {
                    msg_id: None,
                    in_reply_to: None,
                    payload: BroadcastPayload::InitOk {},
                },
            },
            _ => {
                panic!("invalid type")
            }
        }
    }
}
