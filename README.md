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
- An in-memory copy of the new index is made during compaction.
  - The reference borrows values from the index mapping mutably so as not to use twice as much memory when compacting. I think this is a good idea that I will probably use when I try to make compaction happen in a separate thread (assuming I can do so without breaking borrowing rules).

### Issues

If you're working on the PingCAP talent plan and want to discuss the way I've done something, or if you are just curious about my implementation, feel free to make an issue about it.

### Project Structure

The top-level `kvs` directory is included due to a structural mishap when I was learning to use Cargo. I'll leave it that way as an acknowledgement of my imperfections :grimacing:. Mea culpa!
