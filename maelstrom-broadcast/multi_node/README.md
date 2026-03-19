# Challenge #3b: Multi-Node Broadcast 
In this challenge, we’ll build on our Single-Node Broadcast implementation and replicate our messages across a cluster that has no network partitions.

## Specification
Your node should propagate values it sees from broadcast messages to the other nodes in the cluster. It can use the topology passed to your node in the topology message or you can build your own topology.

The simplest approach is to simply send a node’s entire data set on every message, however, this is not practical in a real-world system. Instead, try to send data more efficiently as if you were building a real broadcast system.

Values should propagate to all other nodes within a few seconds.

## Comments
This solution do not use actor model. Solution that use actor model is in 'multi_node_actor' directory.

## Test
```sh
maelstrom test -w broadcast --bin target.tmp/release/multi_node --node-count 5 --time-limit 20 --rate 10
```

## Result
```sh
INFO [2026-03-19 09:11:27,541] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 199,
         :ok-count 199,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 90,
                            :ok-count 90,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 109,
                       :ok-count 109,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 958,
             :recv-count 958,
             :msg-count 958,
             :msgs-per-op 4.81407},
       :clients {:send-count 418, :recv-count 418, :msg-count 418},
       :servers {:send-count 540,
                 :recv-count 540,
                 :msg-count 540,
                 :msgs-per-op 2.7135677},
       :valid? true},
 :workload {:worst-stale (),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 90,
            :stale-count 0,
            :stale (),
            :never-read-count 0,
            :stable-latencies {0 0, 0.5 0, 0.95 0, 0.99 0, 1 0},
            :attempt-count 90,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```
