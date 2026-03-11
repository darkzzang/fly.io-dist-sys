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
