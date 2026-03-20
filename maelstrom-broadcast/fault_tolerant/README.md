# Challenge #3c: Fault Tolerant Broadcast 
In this challenge, we’ll build on our Multi-Node Broadcast implementation, however, this time we’ll introduce network partitions between nodes so they will not be able to communicate for periods of time.

## Specification
Your node should propagate values it sees from broadcast messages to the other nodes in the cluster—even in the face of network partitions! Values should propagate to all other nodes by the end of the test. Nodes should only return copies of their own local values.

## Evaluation
Build your Rust binary as maelstrom-broadcast and run it against Maelstrom with the following command:

```sh
maelstrom test -w broadcast --bin target.tmp/release/fault_tolerant --node-count 5 --time-limit 20 --rate 10 --nemesis partition  
```

This will run a 5-node cluster like before, but this time with a failing network! Fun!

## Result
```sh
INFO [2026-03-20 19:28:39,895] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 201,
         :ok-count 201,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 98,
                            :ok-count 98,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 103,
                       :ok-count 103,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 3353,
             :recv-count 3218,
             :msg-count 3353,
             :msgs-per-op 16.681593},
       :clients {:send-count 422, :recv-count 422, :msg-count 422},
       :servers {:send-count 2931,
                 :recv-count 2796,
                 :msg-count 2931,
                 :msgs-per-op 14.582089},
       :valid? true},
 :workload {:worst-stale ({:element 0,
                           :outcome :stable,
                           :stable-latency 6894,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 2,
                                                     :time 20408833,
                                                     :type :ok,
                                                     :process 1,
                                                     :f :add,
                                                     :value 0},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 2,
                           :outcome :stable,
                           :stable-latency 6584,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 7,
                                                     :time 331171292,
                                                     :type :ok,
                                                     :process 3,
                                                     :f :add,
                                                     :value 2},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 1,
                           :outcome :stable,
                           :stable-latency 6583,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 5,
                                                     :time 180113250,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 1},
                           :last-absent #jepsen.history.Op{:index 132,
                                                           :time 6763238753,
                                                           :type :invoke,
                                                           :process 0,
                                                           :f :read,
                                                           :value nil}}
                          {:element 3,
                           :outcome :stable,
                           :stable-latency 6456,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 11,
                                                     :time 458904750,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 3},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 4,
                           :outcome :stable,
                           :stable-latency 6406,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 13,
                                                     :time 508291417,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 4},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 5,
                           :outcome :stable,
                           :stable-latency 6208,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 15,
                                                     :time 707027292,
                                                     :type :ok,
                                                     :process 1,
                                                     :f :add,
                                                     :value 5},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}
                          {:element 6,
                           :outcome :stable,
                           :stable-latency 5995,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 17,
                                                     :time 767518417,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 6},
                           :last-absent #jepsen.history.Op{:index 132,
                                                           :time 6763238753,
                                                           :type :invoke,
                                                           :process 0,
                                                           :f :read,
                                                           :value nil}}
                          {:element 7,
                           :outcome :stable,
                           :stable-latency 5958,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 19,
                                                     :time 956308834,
                                                     :type :ok,
                                                     :process 3,
                                                     :f :add,
                                                     :value 7},
                           :last-absent #jepsen.history.Op{:index 136,
                                                           :time 6915264045,
                                                           :type :invoke,
                                                           :process 2,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 98,
            :stale-count 49,
            :stale (0
                    1
                    2
                    3
                    4
                    5
                    6
                    7
                    8
                    9
                    10
                    11
                    12
                    13
                    14
                    15
                    16
                    17
                    18
                    19
                    20
                    21
                    22
                    23
                    24
                    25
                    26
                    27
                    28
                    29
                    30
                    31
                    32
                    33
                    34
                    39
                    47
                    48
                    49
                    50
                    51
                    54
                    63
                    66
                    71
                    72
                    89
                    93
                    94),
            :never-read-count 0,
            :stable-latencies {0 0,
                               0.5 8,
                               0.95 6406,
                               0.99 6894,
                               1 6894},
            :attempt-count 98,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
  
``` 
