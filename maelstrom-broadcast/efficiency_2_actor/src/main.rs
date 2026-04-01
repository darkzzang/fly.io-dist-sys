mod actor;

use actor::{Event, NodeActor, StdinActor, StdoutActor};
use maelstrom_protocol::Message;
use std::time::Duration;
use tokio::{sync::mpsc::unbounded_channel, time::interval};

const INTERVAL: u64 = 300;
// Theshold of latency challenge gave.(ms)
const THRESHOLD: u64 = 1000;
// Latency of other node in challenge.(ms)
const LATENCY: u64 = 100;

#[tokio::main]
async fn main() {
    let (tx_event, rx_event) = unbounded_channel::<Event>();
    let (tx_msg, rx_msg) = unbounded_channel::<Message>();

    let stdout_actor = StdoutActor::new(rx_msg);
    tokio::spawn(async move {
        stdout_actor.run_stdout().await;
    });

    let node_actor = NodeActor::new(rx_event, tx_msg);
    tokio::spawn(async move {
        node_actor.run().await;
    });

    let tx_gossip = tx_event.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(INTERVAL));
        loop {
            ticker.tick().await;
            tx_gossip.send(Event::GossipTick).unwrap();
        }
    });

    let tx_anti_entropy = tx_event.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(THRESHOLD - LATENCY));
        loop {
            ticker.tick().await;
            tx_anti_entropy.send(Event::AntiEntropyTick).unwrap();
        }
    });

    let mut stdin_actor = StdinActor::new(tx_event);
    stdin_actor.run_stdin_reader().await;
}
