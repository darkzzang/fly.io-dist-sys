# Challenge #3a: Single-Node Broadcast 
In this challenge, you’ll need to implement a broadcast system that gossips messages between all nodes in the cluster. Gossiping is a common way to propagate information across a cluster when you don’t need strong consistency guarantees.

This challenge is broken up into multiple sections so that you can build out your system incrementally. First, we’ll start with a single-node broadcast system. That may sound like an oxymoron, but it allows us to ensure our message handlers work correctly in isolation before we attempt to share messages between nodes.

## Specification

### `broadcast`
The `message` in this case is an integer. Messages must be stored uniquely and will be returned when a `read` request is received.

* Receive
```json
{
  "type": "broadcast",
  "message": 1000
}
```

* Response
```json
{
  "type": "broadcast_ok"
}
```

### `read`
* Receive
```json
{
  "type": "read"
}
```

* Response
```json
{
  "type": "read_ok",
  "messages": [1, 8, 72, 25]
}
```

### `topology`
This message type won't be used in Challenge #3a.
* Receive
```json
{
  "type": "topology",
  "topology": {
    "n1": ["n2", "n3"],
    "n2": ["n1"],
    "n3": ["n1"]
  }
}
```

* Response
```json
{
  "type": "topology_ok"
}
```


## Test script
```sh
maelstrom test -w broadcast --bin target.tmp/debug/single_node --node-count 1 --time-limit 20 --rate 10          
```

## Result
```sh
INFO [2026-03-16 06:52:15,843] jepsen test runner - jepsen.core Run complete, writing
INFO [2026-03-16 06:52:15,903] jepsen node n0 - maelstrom.db Tearing down n0
INFO [2026-03-16 06:52:17,037] jepsen node n0 - maelstrom.net Shutting down Maelstrom network
INFO [2026-03-16 06:52:17,041] jepsen test runner - jepsen.core Analyzing...
INFO [2026-03-16 06:52:17,235] jepsen test runner - jepsen.core Analysis complete
INFO [2026-03-16 06:52:17,244] jepsen results - jepsen.store Wrote /workspace/store/broadcast/20260316T065145.052Z/results.edn
INFO [2026-03-16 06:52:17,251] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 193,
         :ok-count 193,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 86,
                            :ok-count 86,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 107,
                       :ok-count 107,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 390,
             :recv-count 390,
             :msg-count 390,
             :msgs-per-op 2.0207255},
       :clients {:send-count 390, :recv-count 390, :msg-count 390},
       :servers {:send-count 0,
                 :recv-count 0,
                 :msg-count 0,
                 :msgs-per-op 0.0},
       :valid? true},
 :workload {:worst-stale (),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 86,
            :stale-count 0,
            :stale (),
            :never-read-count 0,
            :stable-latencies {0 0, 0.5 0, 0.95 0, 0.99 0, 1 0},
            :attempt-count 86,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
  
```
