# [Challenge #1: Echo](https://fly.io/dist-sys/1/)

Our first challenge is more of a “getting started” guide" to get the hang of working with Maelstrom in Rust. In [Maelstrom](https://github.com/jepsen-io/maelstrom), we create a node which is a binary that receives JSON messages from STDIN and sends JSON messages to STDOUT. You can find a full [protocol specification](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md) on the Maelstrom project.

### Receive Message
```json
{
  "src": "c1",
  "dest": "n1",
  "body": {
    "type": "echo",
    "msg_id": 1,
    "echo": "Please echo 35"
  }
}
```

### Echo Message
```json
{
  "src": "n1",
  "dest": "c1",
  "body": {
    "type": "echo_ok",
    "msg_id": 1,
    "in_reply_to": 1,
    "echo": "Please echo 35"
  }
}
```

### Running our node in Maelstrom
```sh
maelstrom test -w echo --bin target.tmp/release/maelstrom_echo --node-count 1 --time-limit 10
```

### Debuging Maelstrom
```sh
maelstrom serve
```

### Result

```sh
INFO [2026-03-12 17:08:11,217] jepsen test runner - jepsen.core Run complete, writing
INFO [2026-03-12 17:08:11,260] jepsen node n0 - maelstrom.db Tearing down n0
INFO [2026-03-12 17:08:12,481] jepsen node n0 - maelstrom.net Shutting down Maelstrom network
INFO [2026-03-12 17:08:12,485] jepsen test runner - jepsen.core Analyzing...
INFO [2026-03-12 17:08:12,640] jepsen test runner - jepsen.core Analysis complete
INFO [2026-03-12 17:08:12,647] jepsen results - jepsen.store Wrote /workspace/store/echo/20260312T170800.587Z/results.edn
INFO [2026-03-12 17:08:12,652] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 49,
         :ok-count 49,
         :fail-count 0,
         :info-count 0,
         :by-f {:echo {:valid? true,
                       :count 49,
                       :ok-count 49,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 100,
             :recv-count 100,
             :msg-count 100,
             :msgs-per-op 2.0408163},
       :clients {:send-count 100, :recv-count 100, :msg-count 100},
       :servers {:send-count 0,
                 :recv-count 0,
                 :msg-count 0,
                 :msgs-per-op 0.0},
       :valid? true},
 :workload {:valid? true, :errors ()},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```
