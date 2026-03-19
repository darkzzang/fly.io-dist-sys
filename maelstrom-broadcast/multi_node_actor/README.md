# Challenge #3b: Multi-Node Broadcast 
In this challenge, we’ll build on our Single-Node Broadcast implementation and replicate our messages across a cluster that has no network partitions.

## Specification
Your node should propagate values it sees from broadcast messages to the other nodes in the cluster. It can use the topology passed to your node in the topology message or you can build your own topology.

The simplest approach is to simply send a node’s entire data set on every message, however, this is not practical in a real-world system. Instead, try to send data more efficiently as if you were building a real broadcast system.

Values should propagate to all other nodes within a few seconds.

## Comments
This solution uses actor model. Solution that do not use actor model is in 'multi_node' directory.

## Test
```sh
maelstrom test -w broadcast --bin target.tmp/release/multi_node_actor --node-count 5 --time-limit 20 --rate 10
```

## Result
```sh
INFO [2026-03-19 09:18:33,702] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
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
                            :count 99,
                            :ok-count 99,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 94,
                       :ok-count 94,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 3385,
             :recv-count 3385,
             :msg-count 3385,
             :msgs-per-op 17.53886},
       :clients {:send-count 406, :recv-count 406, :msg-count 406},
       :servers {:send-count 2979,
                 :recv-count 2979,
                 :msg-count 2979,
                 :msgs-per-op 15.435233},
       :valid? true},
 :workload {:worst-stale ({:element 31,
                           :outcome :stable,
                           :stable-latency 191,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 131,
                                                     :time 6879011253,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 31},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 7070126920,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 21,
                           :outcome :stable,
                           :stable-latency 168,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 95,
                                                     :time 5183010544,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 21},
                           :last-absent #jepsen.history.Op{:index 100,
                                                           :time 5351818253,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 92,
                           :outcome :stable,
                           :stable-latency 160,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 353,
                                                     :time 18589187800,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 92},
                           :last-absent #jepsen.history.Op{:index 354,
                                                           :time 18749972509,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 29,
                           :outcome :stable,
                           :stable-latency 155,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 125,
                                                     :time 6656764545,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 29},
                           :last-absent #jepsen.history.Op{:index 126,
                                                           :time 6811932337,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 66,
                           :outcome :stable,
                           :stable-latency 153,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 245,
                                                     :time 13010969048,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 66},
                           :last-absent #jepsen.history.Op{:index 248,
                                                           :time 13164047881,
                                                           :type :invoke,
                                                           :process 4,
                                                           :f :read,
                                                           :value nil}}
                          {:element 45,
                           :outcome :stable,
                           :stable-latency 145,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 177,
                                                     :time 9226794796,
                                                     :type :ok,
                                                     :process 4,
                                                     :f :add,
                                                     :value 45},
                           :last-absent #jepsen.history.Op{:index 178,
                                                           :time 9371940380,
                                                           :type :invoke,
                                                           :process 0,
                                                           :f :read,
                                                           :value nil}}
                          {:element 11,
                           :outcome :stable,
                           :stable-latency 145,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 63,
                                                     :time 3428762918,
                                                     :type :ok,
                                                     :process 1,
                                                     :f :add,
                                                     :value 11},
                           :last-absent #jepsen.history.Op{:index 66,
                                                           :time 3573842085,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 17,
                           :outcome :stable,
                           :stable-latency 144,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 81,
                                                     :time 4483391919,
                                                     :type :ok,
                                                     :process 4,
                                                     :f :add,
                                                     :value 17},
                           :last-absent #jepsen.history.Op{:index 82,
                                                           :time 4627839169,
                                                           :type :invoke,
                                                           :process 0,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 99,
            :stale-count 27,
            :stale (0
                    1
                    6
                    10
                    11
                    12
                    17
                    20
                    21
                    26
                    29
                    30
                    31
                    36
                    45
                    52
                    64
                    66
                    67
                    70
                    72
                    76
                    77
                    78
                    82
                    90
                    92),
            :never-read-count 0,
            :stable-latencies {0 0, 0.5 0, 0.95 153, 0.99 191, 1 191},
            :attempt-count 99,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```
