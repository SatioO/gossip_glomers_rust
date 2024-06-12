use crate::{Body, Message, Payload};

#[derive(Debug, Default)]
pub(crate) struct Node {
    pub(crate) id: String,
    pub(crate) availble_nodes: Vec<String>,
}

impl Node {
    pub(crate) fn init(&self, message: Message<Body>) -> Node {
        match message.body.payload {
            Payload::Init {
                node_id, node_ids, ..
            } => {
                return Node {
                    id: node_id,
                    availble_nodes: node_ids,
                }
            }
            _ => panic!("unknown variant"),
        }
    }
}
