# pna-rust

This is my implementation of PingCAP's [Practical Networking Applications](https://github.com/pingcap/talent-plan/tree/master/rust) project series in Rust. 

In short, the PNA project consists of labs that work towards the development of a "high-performance, networked, parallel and asynchronous key/value store." Tests are provided to help determine correctness while working through these labs. 

### Commentary

I will keep track of how my code differs from the reference for each part, as well as implementation difficulties I run into along the way. I will try not to look at the reference at all until I've passed the test cases with the exception being when I get pretty stuck (which I will note below).

##### Part 1 (in-memory key-value store)
- `structopt` was used instead of `clap` to reduce commandline-related boilerplate and increase clarity (PingCAP-suggested)

##### Part 2 (disk-backed key-value store with compacting log file)
- No 'generations' (epochs) were used to implement log file compaction. 
  - Compaction is instead always done in a temporary file, and that temporary file is moved to overwrite the existing log once compaction is complete. I think this made the code easier to follow while still maintaining the same robustness as the reference.
  
##### Part 3 (networked disk-backed key-value store with multiple engines)
- Simplified read/write benchmarking
  - The instructions for brenchmarking seemed a bit complicated for comparing performance, so I simplified them. I believe these simplified benchmarks will be sufficient for determining relative performance.
- Binary protocol for client/server communication
  - Although I could have used a JSON protocol as was done in the reference, I felt it would probably be best to have a more compact format for sending data over the network.
- Single request per connection
  - Since the CLI mainly expects one-off requests from client to server, I chose to implement a protocol that best fit that use case (yet in a more general setting, I could see why being able to batch requests makes sense). Still, since I was using a binary protocol, I would have to be careful about how I add and parse delimiters in the requests/responses.

### Issues

If you're working on the PingCAP talent plan and want to discuss the way I've done something, or if you are just curious about my implementation, feel free to make an issue about it.
