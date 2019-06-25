use crate::Result;
use std::marker::Sized;

/// Defines interface for a pool of threads
pub trait ThreadPool {
    /// Creates a new thread pool
    ///
    /// # Arguments
    ///
    /// - threads - the number of threads in the pool
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
    /// Spawns the provided job in one of the threads held by the pool
    ///
    /// # Arguments
    ///
    /// - job - the logic to run inside the thread
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

pub use naive::NaiveThreadPool;
pub use rayon::RayonThreadPool;
pub use shared_queue::SharedQueueThreadPool;

mod naive;
mod rayon;
mod shared_queue;
