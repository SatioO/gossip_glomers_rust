use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::packet::{Message, RequestBody};

#[derive(Debug, Default)]
pub struct Node {
    pub id: String,
    pub peers: Vec<String>,
    pub storage: Storage,
}

impl Node {
    pub fn init(message: Message) -> Self {
        match message.body {
            RequestBody::Init {
                node_id, node_ids, ..
            } => {
                return Node {
                    id: node_id,
                    peers: node_ids,
                    storage: Storage::new(),
                }
            }
            _ => panic!("Invalid message type"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Storage {
    pub(crate) messages: Vec<u64>,
    pub(crate) topology: HashMap<String, Vec<String>>,
}

impl Storage {
    pub(crate) fn new() -> Storage {
        Storage::default()
    }

    pub(crate) fn init_topology(&mut self, topology: HashMap<String, Vec<String>>) {
        self.topology = topology;
    }

    pub(crate) fn get_messages(&mut self) -> Vec<u64> {
        self.messages.to_owned()
    }

    pub(crate) fn add_message(&mut self, message: u64) {
        self.messages.push(message);
    }
}
