# Challenge #3c: Fault Tolerant Broadcast 
In this challenge, we’ll build on our Multi-Node Broadcast implementation, however, this time we’ll introduce network partitions between nodes so they will not be able to communicate for periods of time.

## Specification
Your node should propagate values it sees from broadcast messages to the other nodes in the cluster—even in the face of network partitions! Values should propagate to all other nodes by the end of the test. Nodes should only return copies of their own local values.

## Evaluation
Build your Rust binary as maelstrom-broadcast and run it against Maelstrom with the following command:

```sh
maelstrom test -w broadcast --bin target.tmp/release/fault_tolerant_actor --node-count 5 --time-limit 20 --rate 10 --nemesis partition  
```

This will run a 5-node cluster like before, but this time with a failing network! Fun!

## Comment
This source code is same with `mulit_node_actor`.

## Result
```sh
INFO [2026-03-20 19:52:16,660] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 200,
         :ok-count 200,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 87,
                            :ok-count 87,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 113,
                       :ok-count 113,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 3413,
             :recv-count 2270,
             :msg-count 3413,
             :msgs-per-op 17.065},
       :clients {:send-count 420, :recv-count 420, :msg-count 420},
       :servers {:send-count 2993,
                 :recv-count 1850,
                 :msg-count 2993,
                 :msgs-per-op 14.965},
       :valid? true},
 :workload {:worst-stale ({:element 0,
                           :outcome :stable,
                           :stable-latency 19261,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 2,
                                                     :time 19881958,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 0},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 1,
                           :outcome :stable,
                           :stable-latency 19238,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 5,
                                                     :time 43043542,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 1},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 3,
                           :outcome :stable,
                           :stable-latency 18786,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 13,
                                                     :time 495368334,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 3},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 4,
                           :outcome :stable,
                           :stable-latency 18445,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 17,
                                                     :time 836498084,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 4},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 2,
                           :outcome :stable,
                           :stable-latency 18387,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 9,
                                                     :time 293958458,
                                                     :type :ok,
                                                     :process 4,
                                                     :f :add,
                                                     :value 2},
                           :last-absent #jepsen.history.Op{:index 362,
                                                           :time 18681372842,
                                                           :type :invoke,
                                                           :process 1,
                                                           :f :read,
                                                           :value nil}}
                          {:element 5,
                           :outcome :stable,
                           :stable-latency 17632,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 37,
                                                     :time 1649292751,
                                                     :type :ok,
                                                     :process 0,
                                                     :f :add,
                                                     :value 5},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 6,
                           :outcome :stable,
                           :stable-latency 17481,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 39,
                                                     :time 1799662918,
                                                     :type :ok,
                                                     :process 1,
                                                     :f :add,
                                                     :value 6},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}
                          {:element 7,
                           :outcome :stable,
                           :stable-latency 16646,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 53,
                                                     :time 2635125793,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 7},
                           :last-absent #jepsen.history.Op{:index 378,
                                                           :time 19281521842,
                                                           :type :invoke,
                                                           :process 3,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 87,
            :stale-count 81,
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
                    35
                    36
                    37
                    38
                    39
                    40
                    41
                    42
                    43
                    44
                    45
                    46
                    47
                    48
                    49
                    50
                    51
                    52
                    53
                    54
                    55
                    56
                    57
                    58
                    59
                    60
                    61
                    62
                    63
                    64
                    65
                    66
                    67
                    68
                    69
                    70
                    71
                    72
                    73
                    74
                    75
                    76
                    77
                    79
                    80
                    81),
            :never-read-count 0,
            :stable-latencies {0 0,
                               0.5 9244,
                               0.95 18387,
                               0.99 19261,
                               1 19261},
            :attempt-count 87,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ  
```
