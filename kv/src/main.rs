use core::panic;
use std::fmt::Debug;

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::Write;
use tokio::{
    io::AsyncBufReadExt,
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
    pub id: Option<String>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Payload {
    Init {},
    InitOk {},
    Echo { echo: String },
    EchoOk { echo: String },
}

// {"src":"c1","dest":"n1","body":{"type": "init","msg_id":"1"}}
// {"src":"c1","dest":"n1","body":{"type": "echo","msg_id":"1","echo":"Echo Hello World"}}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (writer_tx, mut writer_rx) = channel::<Message<Body>>(1);
    let (reader_tx, mut reader_rx) = channel::<Message<Body>>(1);

    let reader_task = task::spawn(async move { read_from_stdin(reader_tx).await });
    let handler_task = task::spawn(async move { handle_messages(&mut reader_rx, writer_tx).await });
    let writer_task = task::spawn(async move { write_to_stdout(&mut writer_rx).await });

    let _ = tokio::try_join!(reader_task, handler_task, writer_task);

    Ok(())
}

async fn read_from_stdin<T: Debug + DeserializeOwned>(reader_tx: Sender<T>) {
    let stdin = tokio::io::stdin();
    let mut reader = tokio::io::BufReader::new(stdin);

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
    reader_rx: &mut Receiver<Message<Body>>,
    writer_tx: Sender<Message<Body>>,
) {
    loop {
        let message = reader_rx.recv().await.unwrap();
        match message.body.payload {
            Payload::Init {} => {
                writer_tx.send(message).await.unwrap();
            }
            Payload::Echo { echo } => {
                println!("echo: {:?}", echo)
            }
            _ => {
                panic!("invalid type")
            }
        }
    }
}
