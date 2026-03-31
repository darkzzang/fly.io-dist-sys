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
```

```sh
INFO [2026-03-31 07:05:58,345] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 206,
         :ok-count 206,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 96,
                            :ok-count 96,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 110,
                       :ok-count 110,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 4334,
             :recv-count 4334,
             :msg-count 4334,
             :msgs-per-op 21.038836},
       :clients {:send-count 512, :recv-count 512, :msg-count 512},
       :servers {:send-count 3822,
                 :recv-count 3822,
                 :msg-count 3822,
                 :msgs-per-op 18.553398},
       :valid? true},
 :workload {:worst-stale ({:element 69,
                           :outcome :stable,
                           :stable-latency 423,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 237,
                                                     :time 13170594173,
                                                     :type :ok,
                                                     :process 14,
                                                     :f :add,
                                                     :value 69},
                           :last-absent #jepsen.history.Op{:index 242,
                                                           :time 13593901465,
                                                           :type :invoke,
                                                           :process 17,
                                                           :f :read,
                                                           :value nil}}
                          {:element 79,
                           :outcome :stable,
                           :stable-latency 336,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 279,
                                                     :time 15699854966,
                                                     :type :ok,
                                                     :process 9,
                                                     :f :add,
                                                     :value 79},
                           :last-absent #jepsen.history.Op{:index 282,
                                                           :time 16036481383,
                                                           :type :invoke,
                                                           :process 11,
                                                           :f :read,
                                                           :value nil}}
                          {:element 56,
                           :outcome :stable,
                           :stable-latency 318,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 197,
                                                     :time 10647882672,
                                                     :type :ok,
                                                     :process 20,
                                                     :f :add,
                                                     :value 56},
                           :last-absent #jepsen.history.Op{:index 202,
                                                           :time 10966236256,
                                                           :type :invoke,
                                                           :process 23,
                                                           :f :read,
                                                           :value nil}}
                          {:element 90,
                           :outcome :stable,
                           :stable-latency 314,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 325,
                                                     :time 17893838217,
                                                     :type :ok,
                                                     :process 6,
                                                     :f :add,
                                                     :value 90},
                           :last-absent #jepsen.history.Op{:index 328,
                                                           :time 18207841384,
                                                           :type :invoke,
                                                           :process 8,
                                                           :f :read,
                                                           :value nil}}
                          {:element 12,
                           :outcome :stable,
                           :stable-latency 312,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 61,
                                                     :time 2994509919,
                                                     :type :ok,
                                                     :process 4,
                                                     :f :add,
                                                     :value 12},
                           :last-absent #jepsen.history.Op{:index 64,
                                                           :time 3307456919,
                                                           :type :invoke,
                                                           :process 6,
                                                           :f :read,
                                                           :value nil}}
                          {:element 38,
                           :outcome :stable,
                           :stable-latency 294,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 141,
                                                     :time 7503902796,
                                                     :type :ok,
                                                     :process 18,
                                                     :f :add,
                                                     :value 38},
                           :last-absent #jepsen.history.Op{:index 146,
                                                           :time 7797937129,
                                                           :type :invoke,
                                                           :process 21,
                                                           :f :read,
                                                           :value nil}}
                          {:element 18,
                           :outcome :stable,
                           :stable-latency 287,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 85,
                                                     :time 4625379878,
                                                     :type :ok,
                                                     :process 16,
                                                     :f :add,
                                                     :value 18},
                           :last-absent #jepsen.history.Op{:index 92,
                                                           :time 4913012461,
                                                           :type :invoke,
                                                           :process 20,
                                                           :f :read,
                                                           :value nil}}
                          {:element 93,
                           :outcome :stable,
                           :stable-latency 255,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 335,
                                                     :time 18416217218,
                                                     :type :ok,
                                                     :process 11,
                                                     :f :add,
                                                     :value 93},
                           :last-absent #jepsen.history.Op{:index 340,
                                                           :time 18671722218,
                                                           :type :invoke,
                                                           :process 14,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 96,
            :stale-count 58,
            :stale (1
                    3
                    4
                    5
                    6
                    8
                    9
                    10
                    11
                    12
                    13
                    14
                    16
                    18
                    19
                    20
                    24
                    27
                    28
                    29
                    30
                    32
                    33
                    38
                    39
                    40
                    42
                    44
                    45
                    46
                    47
                    48
                    49
                    56
                    57
                    60
                    61
                    62
                    63
                    65
                    66
                    69
                    70
                    71
                    72
                    77
                    78
                    79
                    80
                    81
                    82
                    83
                    88
                    89
                    90
                    91
                    93
                    94),
            :never-read-count 0,
            :stable-latencies {0 0, 0.5 64, 0.95 312, 0.99 423, 1 423},
            :attempt-count 96,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```
You can run maelstrom serve to view results or you can locate your most recent run in the ./store directory.

```sh
maelstrom test -w broadcast --bin target.tmp/release/efficiency_1 --node-count 25 --time-limit 20 --rate 10 --latency 100 --nemesis partition
```

```sh
INFO [2026-03-31 07:07:18,582] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 220,
         :ok-count 220,
         :fail-count 0,
         :info-count 0,
         :by-f {:broadcast {:valid? true,
                            :count 103,
                            :ok-count 103,
                            :fail-count 0,
                            :info-count 0},
                :read {:valid? true,
                       :count 117,
                       :ok-count 117,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 3774,
             :recv-count 3470,
             :msg-count 3774,
             :msgs-per-op 17.154545},
       :clients {:send-count 540, :recv-count 540, :msg-count 540},
       :servers {:send-count 3234,
                 :recv-count 2930,
                 :msg-count 3234,
                 :msgs-per-op 14.7},
       :valid? true},
 :workload {:worst-stale ({:element 0,
                           :outcome :stable,
                           :stable-latency 5491,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 5,
                                                     :time 188598042,
                                                     :type :ok,
                                                     :process 2,
                                                     :f :add,
                                                     :value 0},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 1,
                           :outcome :stable,
                           :stable-latency 5381,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 7,
                                                     :time 298638459,
                                                     :type :ok,
                                                     :process 3,
                                                     :f :add,
                                                     :value 1},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 2,
                           :outcome :stable,
                           :stable-latency 5264,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 11,
                                                     :time 416434417,
                                                     :type :ok,
                                                     :process 5,
                                                     :f :add,
                                                     :value 2},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 4,
                           :outcome :stable,
                           :stable-latency 4949,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 19,
                                                     :time 730957084,
                                                     :type :ok,
                                                     :process 9,
                                                     :f :add,
                                                     :value 4},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 3,
                           :outcome :stable,
                           :stable-latency 4765,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 15,
                                                     :time 636453792,
                                                     :type :ok,
                                                     :process 7,
                                                     :f :add,
                                                     :value 3},
                           :last-absent #jepsen.history.Op{:index 114,
                                                           :time 5401535128,
                                                           :type :invoke,
                                                           :process 5,
                                                           :f :read,
                                                           :value nil}}
                          {:element 5,
                           :outcome :stable,
                           :stable-latency 4754,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 21,
                                                     :time 925972292,
                                                     :type :ok,
                                                     :process 10,
                                                     :f :add,
                                                     :value 5},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 6,
                           :outcome :stable,
                           :stable-latency 4280,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 27,
                                                     :time 1399584209,
                                                     :type :ok,
                                                     :process 13,
                                                     :f :add,
                                                     :value 6},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}
                          {:element 7,
                           :outcome :stable,
                           :stable-latency 4191,
                           :lost-latency nil,
                           :known #jepsen.history.Op{:index 29,
                                                     :time 1488604668,
                                                     :type :ok,
                                                     :process 14,
                                                     :f :add,
                                                     :value 7},
                           :last-absent #jepsen.history.Op{:index 118,
                                                           :time 5680506836,
                                                           :type :invoke,
                                                           :process 7,
                                                           :f :read,
                                                           :value nil}}),
            :duplicated-count 0,
            :valid? true,
            :lost-count 0,
            :lost (),
            :stable-count 103,
            :stale-count 80,
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
                    52
                    54
                    55
                    56
                    57
                    58
                    59
                    62
                    63
                    64
                    68
                    69
                    73
                    74
                    75
                    76
                    77
                    78
                    79
                    80
                    81
                    82
                    87
                    88
                    89
                    90
                    92
                    93
                    94
                    95
                    97
                    98
                    99
                    100
                    102),
            :never-read-count 0,
            :stable-latencies {0 0,
                               0.5 203,
                               0.95 4754,
                               0.99 5381,
                               1 5491},
            :attempt-count 103,
            :never-read (),
            :duplicated {}},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ  
```
