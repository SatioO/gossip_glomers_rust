mod daemon;
mod node;
use std::{
    fmt::Debug,
    io::{stdin, Write},
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

use daemon::Daemon;
use node::{BroadcastNode, Node};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<Payload> {
    msg_id: Option<usize>,

    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

fn broadcast<Payload, N>(node: N)
where
    Payload: Debug + Serialize + DeserializeOwned + Send + 'static,
    N: Node<Payload> + Send + 'static,
{
    let (reader_tx, reader_rx) = mpsc::channel::<Message<Payload>>();
    let (writer_tx, writer_rx) = mpsc::channel::<Message<Payload>>();

    let daemon = Daemon::new();
    daemon.run(move || handle_messages(node, reader_rx, writer_tx));
    daemon.run(move || write_to_stdout(writer_rx));

    read_from_stdin(reader_tx)
}

fn read_from_stdin<Payload>(tx: Sender<Message<Payload>>)
where
    Payload: DeserializeOwned + Debug,
{
    for stdin in stdin().lines() {
        if let Ok(line) = stdin {
            let message = serde_json::from_str::<Message<Payload>>(&line).unwrap();
            tx.send(message).unwrap()
        }
    }
}

fn handle_messages<Payload, N>(
    node: N,
    reader_rx: Receiver<Message<Payload>>,
    writer_tx: Sender<Message<Payload>>,
) where
    Payload: Debug,
    N: Node<Payload>,
{
    while let Ok(input) = reader_rx.recv() {
        let reply = node.step(input);
        writer_tx.send(reply).unwrap();
    }
}

fn write_to_stdout<Payload>(writer_rx: Receiver<Message<Payload>>)
where
    Payload: Debug + Serialize,
{
    loop {
        if let Ok(message) = writer_rx.recv_timeout(Duration::from_millis(500)) {
            let reply = serde_json::to_string(&message).unwrap();

            let mut stdout = std::io::stdout();
            writeln!(stdout, "{}", reply).unwrap();
            stdout.flush().unwrap();
        }
    }
}

fn main() {
    let node = BroadcastNode {};
    broadcast(node);
}
