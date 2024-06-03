use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: RequestBody,
}

impl Message {
    pub(crate) fn parse_message(message: String) -> Message {
        serde_json::from_str(&message).unwrap()
    }

    pub(crate) fn format_message(message: Message) -> String {
        serde_json::to_string(&message).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum RequestBody {
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: u64,
    },
    Topology {
        msg_id: u64,
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {
        msg_id: u64,
        in_reply_to: u64,
    },
    Read {
        msg_id: u64,
    },
    ReadOk {
        msg_id: u64,
        in_reply_to: u64,
        messages: Vec<u64>,
    },
    Broadcast {
        msg_id: u64,
        message: u64,
    },
    BroadcastOk {
        msg_id: u64,
        in_reply_to: u64,
    },
    Error {
        in_reply_to: u64,
        code: u64,
        text: String,
    },
}

pub fn write_to_stdout(message: Message) {
    let mut stdout = std::io::stdout();
    let output = Message::format_message(message);
    writeln!(stdout, "{}", output).unwrap();
    stdout.flush().unwrap();
}
