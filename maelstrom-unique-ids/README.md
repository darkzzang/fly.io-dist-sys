# [Challenge #2: Unique ID Generation](https://fly.io/dist-sys/2/)) 

In this challenge, you’ll need to implement a globally-unique ID generation system that runs against Maelstrom’s unique-ids workload. Your service should be totally available, meaning that it can continue to operate even in the face of network partitions.

### RPC: `generate`

```json
{
  "type": "generate"
}
```


### RPC: `generate ok`

```json
{
  "type": "generate_ok",
  "id": 123
}

```

### Running our node in Maelstrom

```sh
maelstrom test -w unique-ids --bin target.tmp/release/maelstrom_unique_ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```


### Result

```sh
INFO [2026-03-12 16:47:50,413] jepsen test runner - jepsen.core Run complete, writing
INFO [2026-03-12 16:47:50,664] jepsen node n0 - maelstrom.db Tearing down n0
INFO [2026-03-12 16:47:50,664] jepsen node n1 - maelstrom.db Tearing down n1
INFO [2026-03-12 16:47:50,664] jepsen node n2 - maelstrom.db Tearing down n2
INFO [2026-03-12 16:47:52,413] jepsen node n0 - maelstrom.net Shutting down Maelstrom network
INFO [2026-03-12 16:47:52,423] jepsen test runner - jepsen.core Analyzing...
INFO [2026-03-12 16:47:53,333] jepsen test runner - jepsen.core Analysis complete
INFO [2026-03-12 16:47:53,338] jepsen results - jepsen.store Wrote /workspace/store/unique-ids/20260312T164719.656Z/results.edn
INFO [2026-03-12 16:47:53,343] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 18502,
         :ok-count 18502,
         :fail-count 0,
         :info-count 0,
         :by-f {:generate {:valid? true,
                           :count 18502,
                           :ok-count 18502,
                           :fail-count 0,
                           :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 37010,
             :recv-count 37010,
             :msg-count 37010,
             :msgs-per-op 2.0003242},
       :clients {:send-count 37010,
                 :recv-count 37010,
                 :msg-count 37010},
       :servers {:send-count 0,
                 :recv-count 0,
                 :msg-count 0,
                 :msgs-per-op 0.0},
       :valid? true},
 :workload {:valid? true,
            :attempted-count 18502,
            :acknowledged-count 18502,
            :duplicated-count 0,
            :duplicated {},
            :range ["n0-1" "n2-999"]},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```
