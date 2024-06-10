use core::panic;
use std::{
    fmt::Debug,
    io::Write,
    sync::mpsc::{self, Receiver, Sender},
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::task;

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
    let (reader_tx, reader_rx) = mpsc::channel::<Message<Body>>();
    let (writer_tx, writer_rx) = mpsc::channel::<Message<Body>>();

    task::spawn(async move {
        if let Err(err) = StdPrinter::reader(reader_tx) {
            println!("error: {:?}", err);
        }
    });

    task::spawn(async move {
        if let Err(err) = handle_messages(reader_rx, writer_tx) {
            println!("error: {:?}", err);
        }
    });

    if let Err(err) = StdPrinter::writer(writer_rx) {
        println!("error: {:?}", err);
    }

    Ok(())
}

fn handle_messages(
    reader_rx: Receiver<Message<Body>>,
    _writer_tx: Sender<Message<Body>>,
) -> anyhow::Result<()> {
    for message in reader_rx {
        match message.body.payload {
            Payload::Init {} => {
                println!("init");
                _writer_tx.send(message)?;
            }
            Payload::Echo { echo } => {
                println!("echo: {:?}", echo)
            }
            _ => {
                panic!("invalid type")
            }
        }
    }

    Ok(())
}

pub trait IOManager<T>
where
    T: Debug + Serialize + DeserializeOwned + Send,
{
    fn reader(ch: Sender<T>) -> anyhow::Result<()>;
    fn writer(ch: Receiver<T>) -> anyhow::Result<()>;
}

struct StdPrinter;
impl<T> IOManager<T> for StdPrinter
where
    T: Debug + Serialize + DeserializeOwned + Send,
{
    fn reader(ch: Sender<T>) -> anyhow::Result<()> {
        for line in std::io::stdin().lines() {
            let line = line.context("Maelstrom input from STDIN could not be read")?;
            let input = serde_json::from_str::<T>(&line)
                .context("Maelstrom input from STDIN could not be deserialized")?;

            if let Err(_) = ch.send(input) {
                return Ok::<_, anyhow::Error>(());
            }
        }

        Ok(())
    }

    fn writer(ch: Receiver<T>) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();

        for message in ch {
            let data =
                serde_json::to_string(&message).context("Message could not be serialized")?;
            writeln!(stdout, "{}", data).unwrap();
            stdout.flush().unwrap();
        }

        Ok(())
    }
}
