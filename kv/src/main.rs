mod node;
use core::panic;
use std::fmt::Debug;

use anyhow::Context;
use node::Node;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Write;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc::{channel, Receiver, Sender},
    task,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<Body> {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Body {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,

    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

// {"src":"c1","dest":"n1","body":{"type": "init","msg_id":1,"node_id": "n1", "node_ids": ["n1"]}}
// {"src":"c1","dest":"n1","body":{"type": "echo","msg_id":1,"echo":"Echo Hello World"}}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (reader_tx, mut reader_rx) = channel::<Message<Body>>(1);
    let (writer_tx, mut writer_rx) = channel::<Message<Body>>(1);
    let node = Node::default();
    let node = init_node(node, writer_tx.clone()).await;

    let reader_task = task::spawn(async move { read_from_stdin(reader_tx).await });
    let handler_task =
        task::spawn(async move { handle_messages(node, &mut reader_rx, writer_tx).await });
    let writer_task = task::spawn(async move { write_to_stdout(&mut writer_rx).await });

    let _ = tokio::try_join!(reader_task, handler_task, writer_task);

    Ok(())
}

async fn init_node(node: Node, writer_tx: Sender<Message<Body>>) -> Node {
    let stdin = tokio::io::stdin();

    let mut reader = BufReader::new(stdin);
    let mut buf = String::new();

    reader.read_line(&mut buf).await.unwrap();
    let message = serde_json::from_str::<Message<Body>>(&buf)
        .context("Maelstrom input from STDIN could not be deserialized")
        .unwrap();

    let node = node.init(message.clone());

    match message.body.payload {
        Payload::Init { node_id, .. } => {
            let reply = Message {
                src: node_id,
                dest: message.src.clone(),
                body: Body {
                    id: Some(0),
                    in_reply_to: message.body.id,
                    payload: Payload::InitOk {},
                },
            };

            writer_tx.send(reply).await.unwrap();
        }
        _ => (),
    }

    node
}

async fn read_from_stdin<T: Debug + DeserializeOwned>(reader_tx: Sender<T>) {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .await
            .context("Failed to read from stdin")
            .unwrap();
        let input = serde_json::from_str::<T>(&buf)
            .context("Maelstrom input from STDIN could not be deserialized")
            .unwrap();

        reader_tx.send(input).await.unwrap();
    }
}

async fn write_to_stdout<T: Debug + Serialize>(writer_rx: &mut Receiver<T>) {
    let mut stdout = std::io::stdout();

    loop {
        let message = writer_rx.recv().await.unwrap();
        let ser = serde_json::to_string(&message).unwrap();
        writeln!(stdout, "{}", ser).unwrap();
        stdout.flush().unwrap();
    }
}

async fn handle_messages(
    node: Node,
    reader_rx: &mut Receiver<Message<Body>>,
    writer_tx: Sender<Message<Body>>,
) {
    while let Some(message) = reader_rx.recv().await {
        match message.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: node.id.clone(),
                    dest: message.src,
                    body: Body {
                        id: Some(0),
                        in_reply_to: message.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                writer_tx.send(reply).await.unwrap();
            }
            _ => {
                panic!("unknown variant")
            }
        }
    }
}
