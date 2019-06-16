# pna-rust

This is my implementation of PingCAP's [Practical Networking Applications](https://github.com/pingcap/talent-plan/tree/master/rust) project series in Rust. 

In short, the PNA project consists of labs that work towards the development of a "high-performance, networked, parallel and asynchronous key/value store." Tests are provided to help determine correctness while working through these labs. 

Note that I compare my implementation to the provided reference code _once I'm finished_ with each lab to learn best practices that are not mentioned by `clippy`. Additionally, I try to add any extensions that are suggested or that I find interesting. For that reason my code should not exactly match up with the PNA reference code, but will have many similarities.

### Enhancements

I will keep track of any enhancements I add here. I expect that these will usually be the PingCAP-suggested extensions, but who knows!

##### Part 1 (in-memory key-value store)
- `structopt` was used instead of `clap` to reduce commandline-related boilerplate and increase clarity (PingCAP-suggested)

### Issues

If you're working on the PingCAP talent plan and want to discuss the way I've done something, or if you are just curious about my implementation and feel I have documented my implementation poorly etc, feel free to make an issue about it.

### Project Structure

The top-level `kvs` directory is included due to a structural mishap when I was learning to use Cargo. I'll leave it that way as an acknowledgement of my imperfections :grimacing:. Mea culpa!
