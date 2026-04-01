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
maelstrom test -w broadcast --bin target.tmp/release/efficiency_1_actor --node-count 25 --time-limit 20 --rate 10 --latency 100 
```

```sh
INFO [2026-04-01 19:02:12,066] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 221,
         :ok-count 221,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 89,
                            :ok-count 89,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 132,
                       :ok-count 132,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 4334,
             :recv-count 4334,
             :msg-count 4334,
             :msgs-per-op 19.610859},
       :clients {:send-count 542, :recv-count 542, :msg-count 542},
       :servers {:send-count 3792,
                 :recv-count 3792,
                 :msg-count 3792,
                 :msgs-per-op 17.158371},
       :valid? true},
 :workload {:worst-stale ({:element 60,
                           :outcome :stable,
                           :stable-latency 368,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 257,
                                                     :time 12589052839,
                                                     :type :ok,
                                                     :process 24,
                                                     :f :add,
                                                     :value 60},
                           :last-absent #jepsen.history.Op{:index 262,
                                                           :time 12957176548,
                                                           :type :invoke,
                                                           :process 1,
                                                           :f :read,
                                                           :value nil}}
                          {:element 41,
                           :outcome :stable,
                           :stable-latency 362,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 183,
                                                     :time 8983766504,
                                                     :type :ok,
                                                     :process 13,
                                                     :f :add,
                                                     :value 41},
                           :last-absent #jepsen.history.Op{:index 192,
                                                           :time 9345899379,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 15,
                           :outcome :stable,
                           :stable-latency 328,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 75,
                                                     :time 3984824002,
                                                     :type :ok,
                                                     :process 11,
                                                     :f :add,
                                                     :value 15},
                           :last-absent #jepsen.history.Op{:index 84,
                                                           :time 4313247669,
                                                           :type :invoke,
                                                           :process 16,
                                                           :f :read,
                                                           :value nil}}
                          {:element 32,
                           :outcome :stable,
                           :stable-latency 306,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 142,
                                                     :time 6983782462,
                                                     :type :ok,
                                                     :process 19,
                                                     :f :add,
                                                     :value 32},
                           :last-absent #jepsen.history.Op{:index 146,
                                                           :time 7290315920,
                                                           :type :invoke,
                                                           :process 21,
                                                           :f :read,
                                                           :value nil}}
                          {:element 31,
                           :outcome :stable,
                           :stable-latency 306,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 143,
                                                     :time 6984185420,
                                                     :type :ok,
                                                     :process 18,
                                                     :f :add,
                                                     :value 31},
                           :last-absent #jepsen.history.Op{:index 146,
                                                           :time 7290315920,
                                                           :type :invoke,
                                                           :process 21,
                                                           :f :read,
                                                           :value nil}}
                          {:element 53,
                           :outcome :stable,
                           :stable-latency 298,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 229,
                                                     :time 11188629714,
                                                     :type :ok,
                                                     :process 10,
                                                     :f :add,
                                                     :value 53},
                           :last-absent #jepsen.history.Op{:index 234,
                                                           :time 11487230089,
                                                           :type :invoke,
                                                           :process 13,
                                                           :f :read,
                                                           :value nil}}
                          {:element 13,
                           :outcome :stable,
                           :stable-latency 295,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 71,
                                                     :time 3754392210,
                                                     :type :ok,
                                                     :process 9,
                                                     :f :add,
                                                     :value 13},
                           :last-absent #jepsen.history.Op{:index 76,
                                                           :time 4049993668,
                                                           :type :invoke,
                                                           :process 12,
                                                           :f :read,
                                                           :value nil}}
                          {:element 9,
                           :outcome :stable,
                           :stable-latency 295,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 57,
                                                     :time 3248165710,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 9},
                           :last-absent #jepsen.history.Op{:index 64,
                                                           :time 3544157543,
                                                           :type :invoke,
                                                           :process 6,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 89,
            :stale-count 55,
            :stale (0
                    2
                    3
                    4
                    5
                    6
                    9
                    10
                    13
                    14
                    15
                    16
                    17
                    18
                    19
                    21
                    22
                    23
                    31
                    32
                    33
                    34
                    35
                    36
                    37
                    39
                    41
                    42
                    43
                    44
                    50
                    51
                    52
                    53
                    54
                    55
                    56
                    57
                    58
                    60
                    62
                    65
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
                    78
                    82
                    83),
            :never-read-count 0,
            :stable-latencies {0 0, 0.5 88, 0.95 306, 0.99 368, 1 368},
            :attempt-count 89,
            :never-read (),
            :duplicated {}},
 :valid? true}

Everything looks good! ヽ(‘ー`)ノ
```

```sh
maelstrom test -w broadcast --bin target.tmp/release/efficiency_1_actor --node-count 25 --time-limit 20 --rate 10 --latency 100 --nemesis partition
```

```sh
INFO [2026-04-01 19:01:06,763] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 226,
         :ok-count 226,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 95,
                            :ok-count 95,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 131,
                       :ok-count 131,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 3826,
             :recv-count 3434,
             :msg-count 3826,
             :msgs-per-op 16.929203},
       :clients {:send-count 552, :recv-count 552, :msg-count 552},
       :servers {:send-count 3274,
                 :recv-count 2882,
                 :msg-count 3274,
                 :msgs-per-op 14.486726},
       :valid? true},
 :workload {:worst-stale ({:element 0,
                           :outcome :stable,
                           :stable-latency 13797,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 13,
                                                     :time 605989959,
                                                     :type :ok,
                                                     :process 6,
                                                     :f :add,
                                                     :value 0},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 1,
                           :outcome :stable,
                           :stable-latency 13733,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 15,
                                                     :time 669748584,
                                                     :type :ok,
                                                     :process 7,
                                                     :f :add,
                                                     :value 1},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 3,
                           :outcome :stable,
                           :stable-latency 13510,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 18,
                                                     :time 893360000,
                                                     :type :ok,
                                                     :process 9,
                                                     :f :add,
                                                     :value 3},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 2,
                           :outcome :stable,
                           :stable-latency 13509,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 19,
                                                     :time 893780584,
                                                     :type :ok,
                                                     :process 8,
                                                     :f :add,
                                                     :value 2},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 4,
                           :outcome :stable,
                           :stable-latency 13215,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 23,
                                                     :time 1188079334,
                                                     :type :ok,
                                                     :process 11,
                                                     :f :add,
                                                     :value 4},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 5,
                           :outcome :stable,
                           :stable-latency 13124,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 27,
                                                     :time 1308924626,
                                                     :type :ok,
                                                     :process 13,
                                                     :f :add,
                                                     :value 5},
                           :last-absent #jepsen.history.Op{:index 298,
                                                           :time 14432942465,
                                                           :type :invoke,
                                                           :process 19,
                                                           :f :read,
                                                           :value nil}}
                          {:element 6,
                           :outcome :stable,
                           :stable-latency 12711,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 31,
                                                     :time 1692157668,
                                                     :type :ok,
                                                     :process 15,
                                                     :f :add,
                                                     :value 6},
                           :last-absent #jepsen.history.Op{:index 296,
                                                           :time 14403370215,
                                                           :type :invoke,
                                                           :process 18,
                                                           :f :read,
                                                           :value nil}}
                          {:element 7,
                           :outcome :stable,
                           :stable-latency 12536,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 35,
                                                     :time 1896867084,
                                                     :type :ok,
                                                     :process 17,
                                                     :f :add,
                                                     :value 7},
                           :last-absent #jepsen.history.Op{:index 298,
                                                           :time 14432942465,
                                                           :type :invoke,
                                                           :process 19,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 95,
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
                    75
                    76
                    79
                    84
                    86
                    87
                    88
                    89
                    90),
            :never-read-count 0,
            :stable-latencies {0 0,
                               0.5 4383,
                               0.95 13215,
                               0.99 13797,
                               1 13797},
            :attempt-count 95,
            :never-read (),
            :duplicated {}},
 :valid? true}

Everything looks good! ヽ(‘ー`)ノ  
```

You can run maelstrom serve to view results or you can locate your most recent run in the ./store directory.
