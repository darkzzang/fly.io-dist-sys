#  Challenge #3e: Efficient Broadcast, Part II

In this challenge, we’ll make our Efficient, Multi-Node Broadcast implementation even more efficient. Why settle for a fast distributed system when you could always make faster?

## Specification

With the same node count of 25 and a message delay of 100ms, your challenge is to achieve the following performance metrics:

    * Messages-per-operation is below 20
    * Median latency is below 1 second
    * Maximum latency is below 2 seconds

## Evaluation

Build your Rust binary as maelstrom-broadcast and run it against Maelstrom with the same command as before:

```sh
maelstrom test -w broadcast --bin target.tmp/release/efficiency_2 --node-count 25 --time-limit 20 --rate 100 --latency 100 

maelstrom test -w broadcast --bin target.tmp/release/efficiency_2 --node-count 25 --time-limit 20 --rate 100 --latency 100 --nemesis partition

```
