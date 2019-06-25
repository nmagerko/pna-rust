# pna-rust

This is my implementation of PingCAP's [Practical Networking Applications](https://github.com/pingcap/talent-plan/tree/master/rust) project series in Rust. 

In short, the PNA project consists of labs that work towards the development of a "high-performance, networked, parallel and asynchronous key/value store." Tests are provided to help determine correctness while working through these labs. 

### Deviations

I will keep track of how my implementation differs from the reference here. I expect that these will involve the PingCAP-suggested extensions, but they may also include reflections about performance (good or bad) and simplicity.

##### Part 1 (in-memory key-value store)
- `structopt` was used instead of `clap` to reduce commandline-related boilerplate and increase clarity (PingCAP-suggested)

##### Part 2 (disk-backed key-value store with compacting log file)
- No 'generations' (epochs) were used to implement log file compaction. 
  - Compaction is instead always done in a temporary file, and that temporary file is moved to overwrite the existing log once compaction is complete. I think this made the code easier to follow while still maintaining the same robustness as the reference.

### Issues

If you're working on the PingCAP talent plan and want to discuss the way I've done something, or if you are just curious about my implementation, feel free to make an issue about it.
