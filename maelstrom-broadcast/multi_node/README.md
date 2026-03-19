# Challenge #3b: Multi-Node Broadcast 
In this challenge, we’ll build on our Single-Node Broadcast implementation and replicate our messages across a cluster that has no network partitions.

## Specification
Your node should propagate values it sees from broadcast messages to the other nodes in the cluster. It can use the topology passed to your node in the topology message or you can build your own topology.

The simplest approach is to simply send a node’s entire data set on every message, however, this is not practical in a real-world system. Instead, try to send data more efficiently as if you were building a real broadcast system.

Values should propagate to all other nodes within a few seconds.

## Test
```sh
maelstrom test -w broadcast --bin target.tmp/release/multi_node --node-count 5 --time-limit 20 --rate 10
```

## Result
```sh
  
```
