# Challenge #4: Grow-Only Counter 


In this challenge, you’ll need to implement a stateless, grow-only counter which will run against Maelstrom’s g-counter workload.
This challenge is different than before in that your nodes will rely on a sequentially-consistent key/value store service provided by Maelstrom.


## Specification

Your node will need to accept two RPC-style message types: add & read. Your service need only be eventually consistent: given a few seconds without writes, it should converge on the correct counter value.
Please note that the final read from each node should return the final & correct count.


### RPC:`add`
Your node should accept add requests and increment the value of a single global counter. Your node will receive a request message body that looks like this:

```json
{
  "type": "add",
  "delta": 123
}
```

and it will need to return an "add_ok" acknowledgement message:

```json
{
  "type": "add_ok"
}
```


### RPC: `read`
Your node should accept read requests and return the current value of the global counter.
Remember that the counter service is only sequentially consistent. Your node will receive a request message body that looks like this:

```json
{
  "type": "read"
}
```

and it will need to return a "read_ok" message with the current value:

```json
{
  "type": "read_ok",
  "value": 1234
}
```

## Service:`seq-kv`

Maelstrom provides a sequentially-consistent key/value store called seq-kv which has read, write, & cas operations.
The Go library provides a KV wrapper for this service that you can instantiate with NewSeqKV():

```go
node := maelstrom.NewNode()
kv := maelstrom.NewSeqKV(node)
```

The API is as follows:

```sh
func (kv *KV) Read(ctx context.Context, key string) (any, error)
    Read returns the value for a given key in the key/value store. Returns an
    *RPCError error with a KeyDoesNotExist code if the key does not exist.

func (kv *KV) ReadInt(ctx context.Context, key string) (int, error)
    ReadInt reads the value of a key in the key/value store as an int.

func (kv *KV) Write(ctx context.Context, key string, value any) error
    Write overwrites the value for a given key in the key/value store.

func (kv *KV) CompareAndSwap(ctx context.Context, key string, from, to any, createIfNotExists bool) error
    CompareAndSwap updates the value for a key if its current value matches the
    previous value. Creates the key if createIfNotExists is true.

    Returns an *RPCError with a code of PreconditionFailed if the previous value
    does not match. Return a code of KeyDoesNotExist if the key did not exist.
```

## Evaluation
Build your Rust binary as maelstrom-counter and run it against Maelstrom with the following command:

```sh
maelstrom test -w g-counter --bin target.tmp/release/maelstrom_counter --node-count 3 --rate 100 --time-limit 20 --nemesis partition
```

```sh
INFO [2026-05-24 17:47:39,603] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 1814,
         :ok-count 1814,
         :fail-count 0,
         :info-count 0,
         :by-f {:add {:valid? true,
                      :count 578,
                      :ok-count 578,
                      :fail-count 0,
                      :info-count 0},
                :read {:valid? true,
                       :count 1236,
                       :ok-count 1236,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 8698,
             :recv-count 8698,
             :msg-count 8698,
             :msgs-per-op 4.7949286},
       :clients {:send-count 3634, :recv-count 3634, :msg-count 3634},
       :servers {:send-count 5064,
                 :recv-count 5064,
                 :msg-count 5064,
                 :msgs-per-op 2.7916207},
       :valid? true},
 :workload {:valid? true,
            :errors nil,
            :final-reads (1196 1196 1196),
            :acceptable ([1196 1196])},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ  
```

---

```sh
maelstrom test -w g-counter --bin target.tmp/release/maelstrom_counter_actor --node-count 3 --rate 100 --time-limit 20 --nemesis partition
```

```sh
INFO [2026-06-08 11:21:24,307] jepsen test runner - jepsen.core {:perf {:latency-graph {:valid? true},
        :rate-graph {:valid? true},
        :valid? true},
 :timeline {:valid? true},
 :exceptions {:valid? true},
 :stats {:valid? true,
         :count 1442,
         :ok-count 1442,
         :fail-count 0,
         :info-count 0,
         :by-f {:add {:valid? true,
                      :count 484,
                      :ok-count 484,
                      :fail-count 0,
                      :info-count 0},
                :read {:valid? true,
                       :count 958,
                       :ok-count 958,
                       :fail-count 0,
                       :info-count 0}}},
 :availability {:valid? true, :ok-fraction 1.0},
 :net {:all {:send-count 8528,
             :recv-count 8528,
             :msg-count 8528,
             :msgs-per-op 5.914008},
       :clients {:send-count 2890, :recv-count 2890, :msg-count 2890},
       :servers {:send-count 5638,
                 :recv-count 5638,
                 :msg-count 5638,
                 :msgs-per-op 3.9098475},
       :valid? true},
 :workload {:valid? true,
            :errors nil,
            :final-reads (956 956 956),
            :acceptable ([956 956])},
 :valid? true}


Everything looks good! ヽ(‘ー`)ノ
```

This will run a 3-node cluster for 20 seconds and increment the counter at the rate of 100 requests per second. It will induce network partitions during the test.
