#  Challenge #3d: Efficient Broadcast, Part I


In this challenge, we’ll improve on our Fault-Tolerant, Multi-Node Broadcast implementation. Distributed systems have different metrics for success. Not only do they need to be correct but they also need to be fast.

The neighbors Maelstrom suggests are, by default, arranged in a two-dimensional grid. This means that messages are often duplicated en route to other nodes, and latencies are on the order of 2 * sqrt(n) network delays.


## Specification
We will increase our node count to 25 and add a delay of 100ms to each message to simulate a slow network. This could be geographic latencies (such as US to Europe) or it could simply be a busy network.

Your challenge is to achieve the following:

  * Messages-per-operation is below 30
  * Median latency is below 400ms
  * Maximum latency is below 600ms 

Feel free to ignore the topology you’re given by Maelstrom and use your own; it’s only a suggestion. Don’t compromise safety under faults. Double-check that your solution is still correct (even though it will be much slower) with `--nemesis partition`


## Messages-per-operation
In the results.edn file produced by Maelstrom, you’ll find a :net key with information about the number of network messages. The :servers key shows just messages between server nodes, and :msgs-per-op shows the number of messages exchanged per logical operation. Almost all our operations are broadcast or read, in a 50/50 mix.
```sh
:net {:all {:send-count 129592,
            :recv-count 129592,
            :msg-count 129592,
            :msgs-per-op 65.121605},
      :clients {:send-count 4080, :recv-count 4080, :msg-count 4080},
      :servers {:send-count 125512,
                :recv-count 125512,
                :msg-count 125512,
                :msgs-per-op 63.071358}

```
In this example we exchanged 63 messages per operation. Half of those are reads, which require no inter-server messages. That means we sent on average 126 messages per broadcast, between 25 nodes: roughly five messages per node.


## Stable latencies
Under :workload you’ll find a map of :stable-latencies. These are quantiles which show the broadcast latency for the minimum, median, 95th, 99th, and maximum latency request. These latencies are measured from the time a broadcast request was acknowledged to when it was last missing from a read on any node. For example, here’s a system whose median latency was 452 milliseconds:
```sh
:stable-latencies {0 0,
                   0.5 452,
                   0.95 674,
                   0.99 731,
                   1 794},

  
```


## Evaluation
Build your Go binary as maelstrom-broadcast and run it against Maelstrom with the following command:
```sh
maelstrom test -w broadcast --bin target.tmp/release/efficiency_1 --node-count 25 --time-limit 20 --rate 10 --latency 100 

maelstrom test -w broadcast --bin target.tmp/release/efficiency_1 --node-count 25 --time-limit 20 --rate 10 --latency 100 --nemesis partition
```
You can run maelstrom serve to view results or you can locate your most recent run in the ./store directory.
