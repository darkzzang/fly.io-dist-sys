mod actor;
mod node;

use actor::{StdinActor, StdoutActor};
use maelstrom_protocol::Message;
use node::Node;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() {
    let (tx, rx) = unbounded_channel::<Message>();

    let node = Node::new();
    let stdout_actor = StdoutActor::new(rx);

    tokio::spawn(async move {
        stdout_actor.run_stdout().await;
    });

    let node_timer = node.clone();
    let timer_tx = tx.clone();

    tokio::spawn(async move {
        StdinActor::run_gossip_timer(node_timer, timer_tx).await;
    });

    let mut stdin_actor = StdinActor::new(tx, node);
    stdin_actor.run_stdin_reader().await;
}
