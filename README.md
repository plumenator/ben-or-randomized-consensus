Simulator for Ben-Or's Randomized Consensus Algorithm
=====================================================

Usage
-----
```bash
$ # Build
$ cargo build --bin `basename $PWD`
$ # Usage
$ ./target/debug/ben-or-randomized-consensus
Error parsing args: need 5 args
Usage: ./target/debug/ben-or-randomized-consensus <number of nodes> <number of zeros> <number of adversaries> <behavior> <transport type
behavior: correct|crashes|sends_invalid_messages|stops_executing|randomly_adversial
transport type: message_channel
$ # Simulate 11 nodes, half of them starting at 0, with no adversaries
$ ./target/debug/ben-or-randomized-consensus 11 5 0 correct message_channel 2>/dev/null
Process 0: outcome: (Phase: 0, Next: 0)
Process 1: outcome: (Phase: 0, Next: 0)
Process 2: outcome: (Phase: 0, Next: 0)
...
...
Process 10: outcome: (Phase: 8, Next: 1, Decide: 1)
Process 1: outcome: (Phase: 9, Next: 1, Decide: 1)
...
...
$ # Simulate 11 nodes, half of them starting at 0, with 5 (n/2) adversaries each with a random adverserial behavior
$ ./target/debug/ben-or-randomized-consensus 11 5 5 randomly_adversial message_channel 2>/dev/null
Process 0: outcome: (Phase: 0, Next: 0)
Process 1: outcome: (Phase: 0, Next: 0)
Process 2: outcome: (Phase: 0, Next: 0)
...
...
Process 10: outcome: (Phase: 8, Next: 1, Decide: 1)
Process 0: outcome: (Phase: 9, Next: 1, Decide: 1)
...
...
$ # Simulate 11 nodes, half of them starting at 0, with 6 (> n/2) adversaries each with a random adverserial behavior
$ ./target/debug/ben-or-randomized-consensus 11 6 6 randomly_adversial message_channel 2>/dev/null
Process 1: outcome: (Phase: 0, Next: 0)
Process 4: outcome: (Phase: 0, Next: 0)
Process 1: outcome: (Phase: 1, Next: 0)
...
...
Process 8: outcome: (Phase: 10, Next: 1)
Process 9: outcome: (Phase: 10, Next: 0)
...
...
```

Remaining Work
--------------
1. Refactor `step::correct()` so that it does not always choose the decided value for the next phase
1. Adverserial strategy for sending random bytes
1. Implement TCP transport
1. Read the args from a config file
1. Use the `log` crate for logging with serverity instead of writing everything to `stderr`
1. Use `tokio` for lightweight async tasks instead of threads
1. ncurses based frontend to show the process states
1. Return `Result` fallible functions instead of logging and ignoring errors

Reference
---------
"Correctness Proof of Ben-Or’s Randomized Consensus Algorithm" by Marcos Kawazoe Aguilera, Sam Toueg ([link](http://disi.unitn.it/~montreso/ds/syllabus/papers/AguileraToeug-CorrecnessBenOr.pdf))
